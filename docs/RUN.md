# Run ruptured (Phase A)

On any Fedora (or close) box with a compositor:

```bash
sudo dnf install -y rust cargo socat chromium labwc grim wtype
# pointer: wlrctl if packaged, else ydotool + ydotoold
git clone https://github.com/cfpperche/ruptOS.git
cd ruptOS
cargo build -p ruptured --release
./target/release/ruptured
```

In another terminal:

```bash
chmod +x scripts/prompt.sh
./scripts/prompt.sh "open chromium https://example.com"
./scripts/prompt.sh screenshot
./scripts/prompt.sh "click 400 300"
./scripts/prompt.sh "type hello"
```

Frames land in `~/.rupture/frames/latest.png`.

Raw JSONL:

```bash
echo '{"type":"act","id":"1","name":"sight.frame","args":{}}' \
  | socat - UNIX-CONNECT:$HOME/.rupture/rupture.sock

echo '{"type":"act","id":"2","name":"reflex.click","args":{"x":400,"y":300}}' \
  | socat - UNIX-CONNECT:$HOME/.rupture/rupture.sock
```

Known organs: `chromium`, `firefox`, `thunar`, `evince`, `terminal`.
Known acts: `app.launch`, `sight.frame`, `reflex.move`, `reflex.click`, `reflex.type`.

Eyes currently shell out to `grim`. Hands try `wlrctl`, then `ydotool`. Native wlr-screencopy still to come.
