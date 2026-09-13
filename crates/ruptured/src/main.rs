mod acts;
mod protocol;

use std::path::{Path, PathBuf};

use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{UnixListener, UnixStream};
use tracing::{error, info, warn};

use crate::protocol::{Incoming, Outgoing, SteerMode};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("ruptured=info".parse().unwrap()),
        )
        .init();

    let sock = socket_path();
    if let Some(parent) = sock.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let _ = std::fs::remove_file(&sock);

    let listener = UnixListener::bind(&sock).unwrap_or_else(|e| {
        panic!("bind {}: {e}", sock.display());
    });
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = std::fs::set_permissions(&sock, std::fs::Permissions::from_mode(0o660));
    }

    info!(path = %sock.display(), "ruptured listening");

    let mut steer = SteerMode::Agent;

    loop {
        tokio::select! {
            _ = tokio::signal::ctrl_c() => {
                info!("signal, leaving seat");
                let _ = std::fs::remove_file(&sock);
                break;
            }
            accepted = listener.accept() => {
                match accepted {
                    Ok((stream, _)) => {
                        if let Err(e) = handle_client(stream, &mut steer).await {
                            warn!(error = %e, "client ended");
                        }
                    }
                    Err(e) => error!(error = %e, "accept"),
                }
            }
        }
    }
}

fn socket_path() -> PathBuf {
    if let Ok(p) = std::env::var("RUPTURE_SOCK") {
        return PathBuf::from(p);
    }
    if Path::new("/run/rupture").is_dir() {
        return PathBuf::from("/run/rupture/rupture.sock");
    }
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
    PathBuf::from(home).join(".rupture/rupture.sock")
}

async fn handle_client(stream: UnixStream, steer: &mut SteerMode) -> Result<(), std::io::Error> {
    let (reader, mut writer) = stream.into_split();
    let mut lines = BufReader::new(reader).lines();

    while let Some(line) = lines.next_line().await? {
        if line.trim().is_empty() {
            continue;
        }
        let incoming: Incoming = match serde_json::from_str(&line) {
            Ok(v) => v,
            Err(e) => {
                write_out(
                    &mut writer,
                    &Outgoing::Error {
                        id: "parse".into(),
                        message: e.to_string(),
                    },
                )
                .await?;
                continue;
            }
        };
        let replies = handle_incoming(incoming, steer);
        for msg in replies {
            write_out(&mut writer, &msg).await?;
        }
    }
    Ok(())
}

async fn write_out(
    writer: &mut tokio::net::unix::OwnedWriteHalf,
    msg: &Outgoing,
) -> Result<(), std::io::Error> {
    let mut line = msg.to_line().unwrap();
    line.push('\n');
    writer.write_all(line.as_bytes()).await
}

fn handle_incoming(msg: Incoming, steer: &mut SteerMode) -> Vec<Outgoing> {
    match msg {
        Incoming::Steer { id, mode } => {
            *steer = mode;
            vec![
                Outgoing::Ack { id: id.clone() },
                Outgoing::Done {
                    id,
                    text: format!("steer={mode:?}"),
                },
            ]
        }
        Incoming::Act { id, name, args } => run_act(id, name, args, *steer),
        Incoming::Prompt { id, text } => interpret_prompt(id, text, *steer),
    }
}

fn run_act(id: String, name: String, args: serde_json::Value, steer: SteerMode) -> Vec<Outgoing> {
    if steer == SteerMode::Human {
        return vec![Outgoing::NeedWheel {
            id,
            reason: "human has the wheel; injection paused".into(),
        }];
    }
    let result = acts::dispatch(&name, &args);
    vec![
        Outgoing::Ack { id: id.clone() },
        Outgoing::Act {
            id: id.clone(),
            path: result.path,
            name: result.name,
            detail: result.detail,
        },
        Outgoing::Observe {
            id: id.clone(),
            ok: result.ok,
            summary: result.summary.clone(),
        },
        Outgoing::Done {
            id,
            text: result.summary,
        },
    ]
}

/// Phase A: a tiny nerve interpreter so a raw prompt can launch an organ
/// without a brain attached yet. Replaced when the model loop lands.
fn interpret_prompt(id: String, text: String, steer: SteerMode) -> Vec<Outgoing> {
    if steer == SteerMode::Human {
        return vec![Outgoing::NeedWheel {
            id,
            reason: "human has the wheel".into(),
        }];
    }

    let lower = text.to_lowercase();
    let (organ, extra) = if lower.contains("firefox") {
        ("firefox", prompt_url(&text))
    } else if lower.contains("files") || lower.contains("thunar") || lower.contains("folder") {
        ("thunar", vec![])
    } else if lower.contains("pdf") || lower.contains("evince") {
        ("evince", vec![])
    } else if lower.contains("terminal") || lower.contains("foot") {
        ("terminal", vec![])
    } else if lower.contains("chromium")
        || lower.contains("chrome")
        || lower.contains("browser")
        || lower.contains("open ")
        || lower.starts_with("http")
    {
        ("chromium", prompt_url(&text))
    } else {
        return vec![
            Outgoing::Ack { id: id.clone() },
            Outgoing::Error {
                id,
                message: format!(
                    "no brain yet; Phase A understands launch prompts only. got: {text:?}"
                ),
            },
        ];
    };

    let args = serde_json::json!({ "id": organ, "args": extra });
    run_act(id, "app.launch".into(), args, steer)
}

fn prompt_url(text: &str) -> Vec<String> {
    text.split_whitespace()
        .find(|w| w.starts_with("http://") || w.starts_with("https://"))
        .map(|u| vec![u.to_string()])
        .unwrap_or_default()
}
