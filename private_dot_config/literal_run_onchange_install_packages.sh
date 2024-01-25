{{ if eq .chezmoi.os "linux" -}}
#!/bin/sh
sudo dnf install flatpak
sudo dnf install firefox
# korean support
sudo dnf install fcitx5 fcitx5-hangul
sudo dnf install glibc-langpack-ko
localectl set-locale LANG-ko_KR.UTF-8

sudo dnf copr enable wezfurlong/wezterm-nightly
sudo dnf install wezterm

sudo dnf copr enable agriffis/neovim-nightly
sudo dnf install neovim

#rust install
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo install nu

{{ else if eq .chezmoi.os "darwin" -}}
#!/bin/sh
cd ~
brew bundle
{{ end -}}