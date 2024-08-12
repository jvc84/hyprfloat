#!/bin/bash
DIR=$(dirname $( realpath ${0} ))


version=$(hyprctl version | grep version | cut -d '.' -f2)

echo $version
if (($version > 41)); then
  cp $DIR/assets/42/hyprland_42.toml $DIR/Cargo.toml
  cp $DIR/assets/42/client_hyprland_42.rs $DIR/src/client.rs
else
  cp $DIR/assets/41/hyprland_41.toml $DIR/Cargo.toml
  cp $DIR/assets/41/client_hyprland_41.rs $DIR/src/client.rs


  fi


cargo build  --release






