#!/bin/bash

DIR="$(dirname "$( realpath "${0}" )")"
bold=$(tput bold)

function notify() {
  echo "${bold}$1"
  hyprctl notify -1 15000 "rgb(AFD8D8)" "$1" &> /dev/null
}


function copy_config() {
  config_dir="$HOME/.config/hypr/"

  if [ -z "$( find "$config_dir" -name "hf.toml" )" ]; then
    text="File hf.toml copied to ~/.config/hypr/hf.toml"

    cp "$DIR/example/hf.toml" "$config_dir"

    notify "$text"
  fi
}

function copy_binaries() {
  text="Binaries: hfmove, hfopen, hfresize, hftogglefloat copied to /usr/bin/"

  cd "$DIR/target/release/" || exit 1
  sudo cp ./{hfmove,hfopen,hfresize,hftogglefloat} /usr/bin/

  notify "$text"
}


cargo build  --release

copy_config
copy_binaries





