use std::collections::HashMap;
use std::process::Command;

use serde::Deserialize;

use crate::protocol::ActPath;

/// Organs the body may launch by short id.
/// Nerve: never `sh -c` an arbitrary string from a prompt.
fn organs() -> HashMap<&'static str, Organ> {
    let mut m = HashMap::new();
    m.insert(
        "chromium",
        Organ {
            bin: "chromium-browser",
            alt: &["chromium", "google-chrome"],
        },
    );
    m.insert("firefox", Organ { bin: "firefox", alt: &[] });
    m.insert(
        "thunar",
        Organ {
            bin: "thunar",
            alt: &["nautilus"],
        },
    );
    m.insert(
        "evince",
        Organ {
            bin: "evince",
            alt: &["zathura"],
        },
    );
    m.insert(
        "terminal",
        Organ {
            bin: "foot",
            alt: &["kitty", "alacritty", "xterm"],
        },
    );
    m
}

struct Organ {
    bin: &'static str,
    alt: &'static [&'static str],
}

#[derive(Debug, Deserialize)]
pub struct LaunchArgs {
    pub id: String,
    #[serde(default)]
    pub args: Vec<String>,
}

#[derive(Debug)]
pub struct ActResult {
    pub path: ActPath,
    pub name: String,
    pub detail: String,
    pub ok: bool,
    pub summary: String,
}

pub fn dispatch(name: &str, args: &serde_json::Value) -> ActResult {
    match name {
        "app.launch" => match serde_json::from_value::<LaunchArgs>(args.clone()) {
            Ok(a) => launch(a),
            Err(e) => fail("app.launch", format!("bad args: {e}")),
        },
        other => fail(other, format!("unknown act '{other}'")),
    }
}

fn launch(args: LaunchArgs) -> ActResult {
    let table = organs();
    let Some(organ) = table.get(args.id.as_str()) else {
        return fail(
            "app.launch",
            format!("unknown organ '{}'; known: {}", args.id, known_ids()),
        );
    };

    let Some(bin) = resolve_bin(organ) else {
        return fail(
            "app.launch",
            format!("organ '{}' is not installed on this tissue", args.id),
        );
    };

    let extra = args.args.clone();
    let mut cmd = Command::new(&bin);
    cmd.args(&extra);
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        cmd.process_group(0);
    }

    match cmd.spawn() {
        Ok(child) => ActResult {
            path: ActPath::Nerve,
            name: "app.launch".into(),
            detail: format!("{} {}", bin, extra.join(" ")),
            ok: true,
            summary: format!(
                "launched organ '{}' as '{bin}' pid {}",
                args.id,
                child.id()
            ),
        },
        Err(e) => fail("app.launch", format!("spawn '{bin}' failed: {e}")),
    }
}

fn resolve_bin(organ: &Organ) -> Option<String> {
    if which(organ.bin) {
        return Some(organ.bin.to_string());
    }
    for alt in organ.alt {
        if which(alt) {
            return Some((*alt).to_string());
        }
    }
    None
}

fn which(bin: &str) -> bool {
    Command::new("which")
        .arg(bin)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

fn known_ids() -> String {
    let mut ids: Vec<_> = organs().keys().copied().collect();
    ids.sort_unstable();
    ids.join(", ")
}

fn fail(name: &str, summary: String) -> ActResult {
    ActResult {
        path: ActPath::Nerve,
        name: name.into(),
        detail: String::new(),
        ok: false,
        summary,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unknown_organ_fails() {
        let args = serde_json::json!({"id": "not-an-app"});
        let r = dispatch("app.launch", &args);
        assert!(!r.ok);
        assert!(r.summary.contains("unknown organ"));
    }

    #[test]
    fn unknown_act_fails() {
        let r = dispatch("reflex.click", &serde_json::json!({}));
        assert!(!r.ok);
        assert!(r.summary.contains("unknown act"));
    }
}
