# Source

The body runtime is the Rust workspace at the repository root.

```
Cargo.toml                 workspace
crates/ruptured            seat owner daemon
```

```bash
cargo build -p ruptured --release
./target/release/ruptured
```

Socket: `$RUPTURE_SOCK`, else `/run/rupture/rupture.sock` if that directory exists, else `~/.rupture/rupture.sock`.

Phase A implements:

- JSONL prompt channel
- `app.launch` nerve act (allowlisted organs)
- `steer` human | agent
- a temporary prompt interpreter that maps "open chromium https://…" onto `app.launch`

Not yet: frames, virtual pointer, accessibility tree, model loop.
