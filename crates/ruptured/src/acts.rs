use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::Deserialize;

use crate::protocol::ActPath;

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

#[derive(Debug, Deserialize)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Deserialize)]
struct ClickArgs {
    x: i32,
    y: i32,
    #[serde(default = "default_button")]
    button: String,
}

fn default_button() -> String {
    "left".into()
}

#[derive(Debug, Deserialize)]
struct TypeArgs {
    text: String,
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
            Err(e) => fail("app.launch", ActPath::Nerve, format!("bad args: {e}")),
        },
        "sight.frame" => frame(),
        "reflex.move" => match serde_json::from_value::<Point>(args.clone()) {
            Ok(p) => pointer_move(p.x, p.y),
            Err(e) => fail("reflex.move", ActPath::Reflex, format!("bad args: {e}")),
        },
        "reflex.click" => match serde_json::from_value::<ClickArgs>(args.clone()) {
            Ok(c) => pointer_click(c.x, c.y, &c.button),
            Err(e) => fail("reflex.click", ActPath::Reflex, format!("bad args: {e}")),
        },
        "reflex.type" => match serde_json::from_value::<TypeArgs>(args.clone()) {
            Ok(t) => type_text(&t.text),
            Err(e) => fail("reflex.type", ActPath::Reflex, format!("bad args: {e}")),
        },
        other => fail(other, ActPath::Nerve, format!("unknown act '{other}'")),
    }
}

fn launch(args: LaunchArgs) -> ActResult {
    let table = organs();
    let Some(organ) = table.get(args.id.as_str()) else {
        return fail(
            "app.launch",
            ActPath::Nerve,
            format!("unknown organ '{}'; known: {}", args.id, known_ids()),
        );
    };
    let Some(bin) = resolve_bin(organ) else {
        return fail(
            "app.launch",
            ActPath::Nerve,
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
        Ok(child) => ok(
            ActPath::Nerve,
            "app.launch",
            format!("{} {}", bin, extra.join(" ")),
            format!("launched organ '{}' as '{bin}' pid {}", args.id, child.id()),
        ),
        Err(e) => fail("app.launch", ActPath::Nerve, format!("spawn '{bin}' failed: {e}")),
    }
}

fn frame() -> ActResult {
    let grim = first_bin(&["grim"]);
    let Some(grim) = grim else {
        return fail(
            "sight.frame",
            ActPath::Reflex,
            "grim is not on PATH; install grim on the tissue".into(),
        );
    };
    let dir = frames_dir();
    let _ = std::fs::create_dir_all(&dir);
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let path = dir.join(format!("{ts}.png"));
    let latest = dir.join("latest.png");
    let out = Command::new(&grim).arg(&path).output();
    match out {
        Ok(o) if o.status.success() => {
            let _ = std::fs::copy(&path, &latest);
            let bytes = std::fs::metadata(&path).map(|m| m.len()).unwrap_or(0);
            ok(
                ActPath::Reflex,
                "sight.frame",
                path.display().to_string(),
                format!("frame {} bytes at {}", bytes, path.display()),
            )
        }
        Ok(o) => fail(
            "sight.frame",
            ActPath::Reflex,
            format!(
                "grim failed: {}",
                String::from_utf8_lossy(&o.stderr).trim()
            ),
        ),
        Err(e) => fail("sight.frame", ActPath::Reflex, format!("grim: {e}")),
    }
}

fn pointer_move(x: i32, y: i32) -> ActResult {
    if run_wlrctl(&["pointer", "move", &x.to_string(), &y.to_string()]) {
        return ok(
            ActPath::Reflex,
            "reflex.move",
            format!("{x},{y}"),
            format!("pointer moved to {x},{y} via wlrctl"),
        );
    }
    if run_ydotool(&["mousemove", "--absolute", &x.to_string(), &y.to_string()]) {
        return ok(
            ActPath::Reflex,
            "reflex.move",
            format!("{x},{y}"),
            format!("pointer moved to {x},{y} via ydotool"),
        );
    }
    fail(
        "reflex.move",
        ActPath::Reflex,
        "no pointer driver; install wlrctl or ydotool".into(),
    )
}

fn pointer_click(x: i32, y: i32, button: &str) -> ActResult {
    let moved = pointer_move(x, y);
    if !moved.ok {
        return moved;
    }
    let btn = match button {
        "right" | "3" => "right",
        "middle" | "2" => "middle",
        _ => "left",
    };
    if run_wlrctl(&["pointer", "click", btn]) {
        return ok(
            ActPath::Reflex,
            "reflex.click",
            format!("{x},{y} {btn}"),
            format!("clicked {btn} at {x},{y} via wlrctl"),
        );
    }
    let code = match btn {
        "right" => "0xC1",
        "middle" => "0xC2",
        _ => "0xC0",
    };
    if run_ydotool(&["click", code]) {
        return ok(
            ActPath::Reflex,
            "reflex.click",
            format!("{x},{y} {btn}"),
            format!("clicked {btn} at {x},{y} via ydotool"),
        );
    }
    fail(
        "reflex.click",
        ActPath::Reflex,
        "moved but could not click; install wlrctl or ydotool".into(),
    )
}

fn type_text(text: &str) -> ActResult {
    if which("wtype") && Command::new("wtype").arg(text).status().map(|s| s.success()).unwrap_or(false) {
        return ok(
            ActPath::Reflex,
            "reflex.type",
            text.into(),
            format!("typed {} chars via wtype", text.len()),
        );
    }
    if run_ydotool(&["type", text]) {
        return ok(
            ActPath::Reflex,
            "reflex.type",
            text.into(),
            format!("typed {} chars via ydotool", text.len()),
        );
    }
    fail(
        "reflex.type",
        ActPath::Reflex,
        "no keyboard driver; install wtype or ydotool".into(),
    )
}

fn run_wlrctl(args: &[&str]) -> bool {
    which("wlrctl") && Command::new("wlrctl").args(args).status().map(|s| s.success()).unwrap_or(false)
}

fn run_ydotool(args: &[&str]) -> bool {
    which("ydotool") && Command::new("ydotool").args(args).status().map(|s| s.success()).unwrap_or(false)
}

fn frames_dir() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
    PathBuf::from(home).join(".rupture/frames")
}

fn first_bin(names: &[&str]) -> Option<String> {
    names.iter().find(|n| which(n)).map(|s| (*s).to_string())
}

fn resolve_bin(organ: &Organ) -> Option<String> {
    if which(organ.bin) {
        return Some(organ.bin.to_string());
    }
    organ.alt.iter().find(|a| which(a)).map(|s| (*s).to_string())
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

fn ok(path: ActPath, name: &str, detail: String, summary: String) -> ActResult {
    ActResult {
        path,
        name: name.into(),
        detail,
        ok: true,
        summary,
    }
}

fn fail(name: &str, path: ActPath, summary: String) -> ActResult {
    ActResult {
        path,
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
        let r = dispatch("app.launch", &serde_json::json!({"id": "not-an-app"}));
        assert!(!r.ok);
        assert!(r.summary.contains("unknown organ"));
    }

    #[test]
    fn unknown_act_fails() {
        let r = dispatch("net.wifi_connect", &serde_json::json!({}));
        assert!(!r.ok);
        assert!(r.summary.contains("unknown act"));
    }

    #[test]
    fn click_requires_point() {
        let r = dispatch("reflex.click", &serde_json::json!({}));
        assert!(!r.ok);
        assert!(r.summary.contains("bad args"));
    }
}
