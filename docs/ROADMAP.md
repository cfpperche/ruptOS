# Roadmap

## Phase A — a body that obeys

A Fedora + labwc image (or a documented install on existing Fedora) where:

- session autologins as `rupture`
- `ruptured` owns the seat
- a prompt on the socket can open the browser, use a page, download a file, and move that file
- a screen mirror is optional
- ten personal-computer tasks complete without a human touching the desktop keyboard

Landed:

- [x] prompt socket + JSONL protocol sketch
- [x] `app.launch` nerve act with an organ allowlist
- [x] `steer` human | agent
- [x] labwc session files and Fedora kickstart package set

Not landed:

- [ ] wlr-screencopy eyes
- [ ] virtual pointer / keyboard hands
- [ ] download + move file as a verified act
- [ ] model loop (brain)

Done means the tasks above, not a framework diagram.

## Phase B — the body knows itself

- Accessibility tree + nerve verbs for the organs on the image
- Network verbs (`net.*`) so wifi and routes are never reflex
- House memory: downloads, accounts, "the spreadsheet from this week"
- Voice as the same channel with a different codec

## Phase C — the machine as product

- Image that boots into ruptOS. First run is "who are you" + accounts, not partitioning
- N bodies under Tachyon
- Take-the-wheel as a real control
- bootc / image updates so the machine is an appliance

## Not on the path until C is real

- Distro brand, installer aesthetics, ISO marketing
- Policy engine, sandbox between organs, enterprise cockpit
- Multi-agent inside the core loop
- Mascot as intelligence
- Wrapping vendor CLIs so we can demo faster
