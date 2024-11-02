#!/bin/bash

DIR="$(dirname "$( realpath "${0}" )")"
bold=$(tput bold)

function notify() {
  echo "${bold}$1"
  hyprctl notify -1 15000 "rgb(AFD8D8)" "$1" &> /dev/null
}


function copy_config() {
  config_dir="$HOME/.config/hyprfloat/"

  mkdir -p $config_dir 
 

  if [ -z "$( find "$config_dir" -name "hf.toml" )" ]; then
    text="File hf.toml copied to $config_dir"

    cp "$DIR/example/hf.toml" "$config_dir" || (echo "${bold}Config was not copied!" && exit 1)

    notify "$text"
  fi
}

function copy_binaries() {
  text="Binaries: hfmovewindow, hfopen, hfresizeactive, hftogglefloating copied to /usr/bin/"

  cd "$DIR/target/release/" || (echo "${bold} Cannot enter '$DIR/target/release/'" && exit 1)
  
  sudo rm /usr/bin/{hfmovewindow,hfopen,hfresizeactive,hftogglefloating} 2> /dev/null
  sudo cp ./{hfmovewindow,hfopen,hfresizeactive,hftogglefloating} /usr/bin/ || (echo "${bold}Binaries was not copied!" && exit 1)
  
  notify "$text"
}


cargo build --release || (echo "${bold}Build Error!" && exit 1)

copy_config
copy_binaries





