# Tissue: Fedora + labwc

Linux is tissue. We still have to pick one, because a body that boots has to boot *something*.

## Decision

**Fedora current stable** as the userspace and kernel.  
**labwc** (wlroots) as the compositor the agent owns.  
**NetworkManager + iproute2 + nftables + modern userland network tools** as the network organ kit.

This is reversible at the image layer. It is not reversible if we pretend the distro *is* the product.

## Why Fedora, not Arch, not Ubuntu, not a custom ISO brand

| Candidate | Fits | Cost |
|---|---|---|
| **Fedora** | Wayland-first, PipeWire default, current kernel, NetworkManager is first-class, image and bootc story, packages for a11y / wlroots / browsers / net tools without heroics | Slightly faster cadence than LTS; we pin a release per image |
| Arch / CachyOS / Omarchy | Newest packages, Hyprland culture, agent CLIs already in PATH | Rolling is a moving floor under a body we want to *know*; Omarchy still treats a human as the desktop user |
| Ubuntu LTS | Long support, familiar packaging | Snaps and older compositor defaults fight a clean seat; slower kernel |
| Debian stable | Predictable | Too old for the Wayland protocols we need without backports |
| From-scratch kernel | Fantasy of purity | We would spend the decade on Wi-Fi, NVIDIA, and printers |

Fedora is the least romantic answer and the best body shop: a current kernel, a compositor we can dominate, and a package set that already contains every organ we need to attach.

When the body is real we may freeze it as a **bootc / image-based** Fedora so the machine updates as an appliance, not as a pet. That is Phase C. Phase A is a Fedora workstation-shaped image that autologins into labwc as user `rupture`.

## Why labwc, not GNOME, not Hyprland

- GNOME is a human desktop with opinions. We would spend months fighting the session.
- Hyprland is a human tiling weapon. Beautiful, noisy, not session-shaped.
- **labwc** is a small wlroots Openbox-like compositor. It speaks the protocols the body needs: screencopy, virtual pointer, virtual keyboard, layer shell. It can run with almost no panels. That emptiness is a feature. The agent does not want a dock.

Sway is an acceptable twin if labwc lags on a protocol. The constraint is **wlroots + seat control**, not the window-management religion.

## Session

Boot → greetd or autologin → labwc as user `rupture` → `ruptured` starts with the session and takes the seat.

There is no GNOME Settings for a human. Network, audio, display, and packages are organs `ruptured` drives.

## Network organs

An agent that cannot operate a network is a demo. The tissue must include tools for *every common situation*, used through nerve verbs first and raw CLI only when a verb does not exist.

### Always on the image

| Job | Tooling |
|---|---|
| Device and link | `iproute2` (`ip`, `ss`, `bridge`), `ethtool` |
| Association | NetworkManager (`nmcli`, `nmtui` unused by humans), `iwd` as Wi-Fi backend where it wins |
| DNS | `systemd-resolved` + `dig` (`bind-utils`) |
| Firewall | `nftables`, `firewalld` (agent talks verbs, not a GUI) |
| Capture / debug | `tcpdump`, `tshark` / wireshark-cli, `nmap`, `mtr`, `iperf3` |
| VPN / overlay | `wireguard-tools`, NetworkManager VPN plugins as needed |
| TLS / HTTP | `curl`, `wget`, `openssl` |
| Sockets as a human would | `socat`, `netcat` |
| Containers / lab | `podman` (rootless), not Docker-as-religion |

### Nerve verbs we owe the brain (Phase B)

```
net.status              links, addresses, routes, DNS, NM connectivity
net.wifi_scan           SSIDs
net.wifi_connect        SSID + secret via NM, never via wpa_supplicant soup
net.wifi_disconnect
net.route               default and specific
net.dns_lookup
net.listen              who is bound
net.http_get            when a browser is overkill
net.allow / net.deny    firewalld service or port
```

Reflex on a Network Settings GUI is failure. If we find the agent clicking GNOME Settings to join Wi-Fi, the nerve is unfinished.

## Other organs on the image

See [image/PACKAGES.md](../image/PACKAGES.md).

Minimum graphical organs:

- Chromium or Firefox (the most important organ in the body)
- a files surface the agent can see (Thunar or Nautilus — pick one and keep it)
- a PDF viewer
- PipeWire + wireplumber
- grim / slurp only as debug helpers; production sight is `ruptured` screencopy
- AT-SPI / at-spi2-core so GTK and a few friends expose a tree

## What we do not put on the image

- A panel, a dock, a human launcher
- Vendor agent CLIs as the runtime (`claude`, `codex`, `pi`, …)
- Flatpak sprawl without a reason
- NVIDIA proprietary dance in Phase A (document it, do not block the body on it)
