# Body

The body is how the brain touches the world.

## Two paths

### Nerve

Stable verbs with typed arguments and typed observations.

Examples:

- `app.launch id=chromium args=[url]`
- `app.focus id=…`
- `a11y.tree` / `a11y.click role=button name=Submit`
- `fs.read` / `fs.write` / `fs.list`
- `clip.read` / `clip.write`
- `net.wifi_connect ssid=…`
- `sh.run` as an escape hatch, capped and logged

Nerve is fast, cheap, and inspectable. It is also incomplete. Banks, Electron canvases, and custom widgets will lie or go silent.

### Reflex

The universal path.

1. Grab a frame (or a focused-window crop).
2. Let the brain point: click, drag, type, scroll, wait.
3. Grab another frame. Diff. Decide if the world moved.

Reflex is slow, expensive, and the only way through a painted UI. Use it when nerve returns empty or contradicts the frame.

## Act budget

One act per step. Verify before the next act. A burst of clicks is how you click "Delete" on the wrong dialog.

## Verification

Every act returns an observation:

```json
{
  "ok": true,
  "path": "nerve",
  "summary": "chromium window focused, url bar shows example.com",
  "tree_hash": "…",
  "frame_id": null
}
```

If `ok` is false, the brain gets the failure and the new perception. It does not invent success in prose.

## Taking the wheel

Some worlds refuse agents: CAPTCHA, SMS 2FA, a hardware key. The body emits `need_wheel`. Input injection pauses. The owner uses the mirror plus local input. When they send `steer: agent`, hands resume.

This is not nostalgia for Linux. It is the world as it is.

## Identity of a machine

Cookies, keyrings, files, wifi history, mail profiles — that is the self of a Corpus. Without it the machine is an internet cafe with an LLM. Phase B writes this down as memory. Phase A at least keeps a persistent home for user `rupture`.
