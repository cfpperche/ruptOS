# ruptOS

**The computer is the agent.**

ruptOS (Rupture) is a personal computer whose only first-class contract with a human is language. You send a prompt. The machine acts on the real desktop, the real network, the real files. There is no Linux user persona. There is no desktop for humans to drive. Linux is tissue. The system session belongs to the agent.

- Brain: a pluggable LLM
- Body: the machine (display, input, apps, disk, network)
- Interface: prompt (text now, voice later)
- Sight: the screen exists for the agent to see
- Optional mirror: watch over the agent's shoulder

This repository is the product source: body runtime, image definition, prompt channel, and documentation. Official project language is **English**.

## Thesis

Three sentences we do not negotiate:

1. The prompt is the computer's API.
2. The screen exists for the agent to see, not for a human to drive.
3. Linux, if present, is tissue. Not product. Not persona. Not UI.

Computer use is not a plugin. It is how the OS exists. The body has two paths:

- **Nerve** — stable verbs: launch app by id, read the accessibility tree, files, clipboard, network.
- **Reflex** — see a frame, move the pointer, type, wait for the screen to change, when the tree lies or the app is a canvas.

Nerve first. Reflex when nerve fails.

## Tissue

We do not ship a new kernel. We ship a body on a known compositor.

| Layer | Choice | Why |
|---|---|---|
| Tissue distro | **Fedora** (current stable) | Modern kernel, Wayland-first, PipeWire, NetworkManager, huge package set, image-friendly |
| Compositor | **labwc** (wlroots) | Screencopy + virtual pointer protocols, small, session-shaped |
| Seat owner | `ruptured` | Sees frames, injects input, launches organs, owns the loop |
| Prompt channel | Unix socket + remote client | Phone / web / Tachyon all dump into one inbound channel |

See [docs/DISTRO.md](docs/DISTRO.md) for the distro decision and the network toolkit.

## Repository map

```
ruptOS/
  README.md
  docs/
  crates/ruptured/          seat owner (Rust)
  image/                    Fedora kickstart + labwc session
  scripts/prompt.sh         send one prompt to the socket
  LICENSE
```

Start here:

- [docs/VISION.md](docs/VISION.md) — product object
- [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) — brain, body, channel
- [docs/DISTRO.md](docs/DISTRO.md) — why Fedora, compositor, network organs
- [docs/BODY.md](docs/BODY.md) — perceive → act → verify; nerve vs reflex
- [docs/RUN.md](docs/RUN.md) — build and talk to ruptured today
- [docs/ROADMAP.md](docs/ROADMAP.md) — phases A–C
- [docs/CONVENTIONS.md](docs/CONVENTIONS.md) — language, names, what we refuse

## Status

Phase A is in progress. `ruptured` listens on a Unix socket and can launch allowlisted organs (`app.launch`). There is no vision, no virtual pointer, and no model loop yet. A prompt such as `open chromium https://example.com` is mapped onto that one nerve act.

```bash
cargo build -p ruptured --release
./target/release/ruptured
./scripts/prompt.sh "open chromium https://example.com"
```

## What this is not

- Not a Linux distro brand that puts a chatbot in the panel
- Not a wrapper around Claude Code, Codex, Pi, or any vendor agent CLI
- Not an IDE
- Not an enterprise policy OS
- Not a mascot runtime

## License

MIT. See [LICENSE](LICENSE).
