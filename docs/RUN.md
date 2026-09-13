# Run ruptured (Phase A)

On any Fedora (or close) box with a compositor:

```bash
sudo dnf install -y rust cargo socat chromium labwc
git clone https://github.com/cfpperche/ruptOS.git
cd ruptOS
cargo build -p ruptured --release
./target/release/ruptured
```

In another terminal:

```bash
chmod +x scripts/prompt.sh
./scripts/prompt.sh "open chromium https://example.com"
```

Or raw JSONL:

```bash
echo '{"type":"act","id":"1","name":"app.launch","args":{"id":"chromium","args":["https://example.com"]}}' \
  | socat - UNIX-CONNECT:$HOME/.rupture/rupture.sock
```

Take the wheel:

```bash
echo '{"type":"steer","id":"2","mode":"human"}' | socat - UNIX-CONNECT:$HOME/.rupture/rupture.sock
```

Known organs: `chromium`, `firefox`, `thunar`, `evince`, `terminal`.

This is not computer-use yet. It is the first nerve act and the channel the brain will speak.
