# ruptOS Phase A tissue. Fedora, labwc seat, network organs.
# Build with livemedia-creator or adapt into a bootc Containerfile later.
# This is not a branded installer. It is a body shop recipe.

# Use Fedora current stable when invoking the builder.
# Example:
#   virt-install / livemedia-creator --ks image/kickstart.ks

text
lang en_US.UTF-8
keyboard us
timezone UTC --utc

rootpw --lock
user --name=rupture --groups=wheel --plaintext --password=rupture

autologin --user=rupture

firewall --enabled --service=ssh
network --bootproto=dhcp --device=link --activate --onboot=on
services --enabled=NetworkManager,sshd,firewalld

%packages
@core
labwc
greetd
at-spi2-core
pipewire
wireplumber
pipewire-pulseaudio
grim
chromium
thunar
evince
foot
NetworkManager
NetworkManager-wifi
iwd
iproute
ethtool
bind-utils
nftables
firewalld
tcpdump
wireshark-cli
nmap
mtr
iperf3
wireguard-tools
curl
wget
openssl
socat
nmap-ncat
podman
ripgrep
jq
git
rust
cargo
%end

%post --erroronfail
install -d -m 0755 /run/rupture /var/lib/rupture/sessions /var/lib/rupture/memory
install -d -m 0755 /home/rupture/.config/labwc /home/rupture/.rupture
# Session files are copied from the repo at image bake time.
# Placeholders so a raw kickstart still boots a compositor.
cat >/home/rupture/.config/labwc/autostart <<'EOF'
mkdir -p /run/rupture "$HOME/.rupture"
command -v ruptured >/dev/null && exec ruptured || exec foot
EOF
chown -R rupture:rupture /home/rupture /var/lib/rupture
%end
