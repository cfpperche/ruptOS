# Image package set

Tissue: Fedora current stable. This list is the organ kit for Phase A. Exact NEVRAs pin when we build an image.

## Session and body

- `labwc`
- `greetd` (or SDDM autologin if greetd fights us)
- `wlroots` stack as pulled by labwc
- `at-spi2-core` `at-spi2-atk`
- `pipewire` `wireplumber` `pipewire-pulseaudio`
- `grim` (debug frames only)

## Sight and input development

- `gcc` `make` `pkgconf` `wayland-devel` `wayland-protocols-devel`
- `libinput-devel` plus whatever virtual-pointer protocol labwc exposes
- `rust` `cargo` once `ruptured` lands in-tree

## The important organ

- `chromium` (preferred) or `firefox`
- `thunar` or `nautilus` — pick one in the first image and do not ship both
- `evince` or `zathura` for PDF

## Network — every common situation

- `NetworkManager` `NetworkManager-wifi` `NetworkManager-tui`
- `iwd` (Wi-Fi backend)
- `iproute` `ethtool`
- `bind-utils` (`dig`, `nslookup`)
- `nftables` `firewalld`
- `tcpdump` `wireshark-cli` `nmap` `mtr` `iperf3`
- `wireguard-tools`
- `curl` `wget` `openssl`
- `socat` `nmap-ncat`
- `podman`

## Files and mundane tools the nerve will wrap

- `coreutils` `findutils` `ripgrep` `jq` `tar` `unzip` `git`

## Explicitly not on the image

- GNOME Shell, KDE Plasma, Hyprland
- `claude`, `codex`, `pi`, or any vendor agent CLI
- Docker Engine as a default (Podman is enough)
