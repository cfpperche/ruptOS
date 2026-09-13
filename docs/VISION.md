# Vision

ruptOS is a computer you speak to.

A conventional OS assumes a human sits in a seat: windows, pointers, settings panels, a shell prompt. An agent bolted onto that world is a guest. ruptOS inverts the seat. The agent *is* the session. The human owns the channel, not the desktop.

## The object

You do not open the computer. You charge it with work.

"Download the invoice from this mailbox and put a PDF on the desk."  
"Join the wifi named X."  
"Fill this form and submit it."  
"Open the bank site and tell me the balance."

The machine uses real applications, a real browser, a real network stack. When an app exposes a verb, it uses the verb (nerve). When the app is a painted canvas or a hostile web page, it looks at the pixels and drives input (reflex).

## Why this is not Habitat

An earlier sketch (Habitat) treated Linux as a home for a coding harness: daemon, typed `os.pkg` tools, TUI first. That is a strong *nerve*. It is the wrong product.

The product is the body. Tools of the operating system remain inside the body as shortcuts. They are not the interface. A human never sees `dnf` or `systemctl`. A human sees "done" and, if they asked, a mirror of the screen.

## Why this is not a vendor CLI

Claude Code, Codex, Pi, OpenCode and the rest are harnesses a developer launches in a project directory. ruptOS is not launched inside a repo. It *is* the machine. We write the loop. We do not wrap someone else's agent process as the runtime.

## The human

- Speaks on the prompt channel (text, later voice).
- May watch a screen mirror.
- May take the wheel for a few minutes when the world is hostile (CAPTCHA, 2FA, a site that hates bots), then give it back.
- Does not administer Fedora.

## Related surfaces

- **Tachyon** may send prompts to N bodies. It does not replace a body.
- **Pico / PiCode**, if they appear, are a face on the mirror. They are not the loop.
