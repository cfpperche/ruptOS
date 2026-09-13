# src/

Body runtime lives here.

Phase A target: `ruptured` — a single long-lived process that

- sits on the labwc seat
- captures frames
- injects pointer and keyboard
- launches organs
- speaks the prompt socket in [docs/ARCHITECTURE.md](../docs/ARCHITECTURE.md)

Language for the daemon is **Rust**, unless a spike proves a wlroots binding is unblockable faster in C. The spike is allowed. Shipping a Node harness as the seat owner is not.

No code in this directory yet.
