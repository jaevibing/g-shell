#!/bin/bash

# setup config files and directories in home directory
mkdir $HOME/.gsh
mkdir $HOME/.gsh/bin
touch $HOME/.gsh_profile
touch $HOME/.gshrc
touch $HOME/.gsh_history

# download latest gsh binary to .gsh
cd $HOME/.gsh/bin
curl -s https://api.github.com/repos/jaevibing/g-shell/releases/latest \
| grep "gsh" \
| cut -d : -f 2,3 \
| tr -d \" \
| wget -qi -
export GSH="$HOME/.gsh"

# check if user can sudo, stolen directly from ohmyzsh's install.sh
user_can_sudo() {
  command_exists sudo || return 1
  case "$PREFIX" in
  *com.termux*) return 1 ;;
  esac
  ! LANG= sudo -n -v 2>&1 | grep -q "may not run sudo"
}

# add .gsh/bin to path
export $PATH="$GSH/bin:$PATH"

# change shell with chsh, force sudo if can sudo = true
if user_can_sudo; then
    sudo -k chsh "$GSH/bin/gsh" "$USER"
else
    chsh -s "$GSH/bin/gsh" "$USER"
fi

echo "g-shell installed successfully!"
