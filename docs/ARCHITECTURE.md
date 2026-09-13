# Architecture

Four layers. None of them is a market agent CLI.

```
[ owner ]
    |
    |  prompt channel  (phone, web, Tachyon, later voice)
    v
[ ruptured ]          seat owner, loop, memory, organs
    |
    +-- nerve: launch, a11y tree, files, clipboard, net verbs
    +-- reflex: frame, pointer, keyboard, wait-for-change
    v
[ labwc session ]     Wayland seat the agent owns
    v
[ Fedora tissue ]     kernel, PipeWire, NetworkManager, packages
```

## Organs

| Organ | Role |
|---|---|
| Brain | Pluggable model + perceive-act-verify loop |
| Eyes | Framebuffer / wlr-screencopy + accessibility tree |
| Hands | Virtual pointer, virtual keyboard, focus, clipboard |
| Ear | Inbound prompt channel |
| Memory | Disk of this machine: sessions, preferences, where things live |
| Tissue | Kernel, compositor, NetworkManager, the packaged apps |

## Process shape

`ruptured` is a long-lived process started with the graphical session (autologin into the `rupture` seat). It:

1. Owns the Wayland seat tools (screencopy, virtual input).
2. Listens on a Unix socket (and later a remote front door) for prompts.
3. Runs one act at a time. No blind burst of forty clicks.
4. Writes session JSONL under `/var/lib/rupture/sessions/`.
5. Never hands tool execution to a child vendor CLI.

Clients (phone app, thin web console, Tachyon) do not execute tools. They send prompts and receive events.

## Loop

Each turn of the computer:

1. **Perceive** — focused app, a11y tree (if any), optional frame or frame-diff, clipboard, recent downloads, last prompt.
2. **Intend** — the brain picks one act: a nerve verb or a reflex gesture, or it answers in language.
3. **Act** — the body performs that one act.
4. **Verify** — new tree / new frame. Did the button vanish? Did the file land in Downloads?
5. **Retry or speak** — if it missed, another act, not an essay.

Perception is lazy. Tree first. Frame when the tree fails. Diff when the frame is large. Vision on every frame is how you go broke.

## Prompt protocol (v0 sketch)

Unix socket `~/.rupture/rupture.sock` (system path `/run/rupture/rupture.sock` on the image).

Inbound JSON lines:

```json
{"type":"prompt","id":"…","text":"Join wifi Harbor and tell me the IP."}
{"type":"steer","id":"…","mode":"human"}
{"type":"steer","id":"…","mode":"agent"}
```

Outbound JSON lines:

```json
{"type":"ack","id":"…"}
{"type":"act","id":"…","path":"nerve|reflex","name":"net.wifi_connect","detail":"Harbor"}
{"type":"observe","id":"…","ok":true,"summary":"associated, ipv4 192.168.0.14"}
{"type":"done","id":"…","text":"Connected to Harbor. Address is 192.168.0.14."}
{"type":"need_wheel","id":"…","reason":"2FA on screen"}
```

Exact schemas land with `ruptured`. This file is the shape.

## Providers

The loop speaks a normalized tool-call + vision interface. Adapters for OpenAI-compatible endpoints, Anthropic, xAI, Google, and Ollama. No provider owns the runtime.
