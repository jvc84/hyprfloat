#!/bin/bash

DIR="$(dirname "$( realpath "${0}" )")"
bold=$(tput bold)

function print_message() {
  printf "\n  -> ${bold}%s$1\n\n"
}

function notify() {
  print_message "$1"
  hyprctl notify -1 15000 "rgb(AFD8D8)" "$1" &> /dev/null
}

function exit_message() {
  print_message "$1"
  hyprctl notify -1 15000 "rgb(FF0000)" "$1" &> /dev/null
  exit 1
}

function copy_config() {
  config_dir="$HOME/.config/hyprfloat/"

  mkdir -p "$config_dir"

  if [ -z "$( find "$config_dir" -name "hf.toml" )" ]; then
    cp "$DIR/example/hf.toml" "$config_dir" || exit_message "Config was not copied!"

    notify "File hf.toml was copied to $config_dir"
  fi
}

function copy_binaries() {
  cd "$DIR/target/release/" || exit_message "Cannot enter '$DIR/target/release/'"
  
  sudo cp ./{hfmovewindow,hfopen,hfresizeactive,hftogglefloating} /usr/bin || exit_message "Binaries was not copied!"
  
  notify "Binaries: hfmovewindow, hfopen, hfresizeactive, hftogglefloating was copied to /usr/bin"
}


function link_config() {
  read -r -p "Link config file 'hf.toml' to '~/.config/hypr/' directory? [Y/n]: " answer

  if [ "$(echo "$answer" | awk '{print tolower($0)}')" == "y" ] || [ "$answer" == "" ]; then
    ln -s "$HOME/.config/hyprfloat/hf.toml" "$HOME/.config/hypr/hf.toml" || exit_message "Cannot link file"

    notify "Config file '$HOME/.config/hyprfloat/hf.toml' was linked to '$HOME/.config/hypr/hf.toml'"
  fi
}


cargo build --release || exit_message "Build Error!"

copy_config
copy_binaries
link_config




