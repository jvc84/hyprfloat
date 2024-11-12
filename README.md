# Hyprfloat

Improve your experience with floating windows!

Supports Hyprland `>=42.0`

# hfopen

https://github.com/user-attachments/assets/e70fcc55-a50f-483f-a1ec-5589ac08de68

## Example `hyprland.conf`:
```
bind = Super Shift, Return, exec, hfopen -o -w 600 -h 450 -p cursor kitty
bind = Super Shift, R, exec, hfopen -o -w 700 -h 650 -p center "kitty ranger"
```

# hftogglefloating

https://github.com/user-attachments/assets/6816a7fa-ec8d-48fa-9fc5-b21af640e069

**Example** `hyprland.conf`:
```
bind = Super Shift, Space, exec, hftogglefloating -p center
bind = Super, Space, exec, hftogglefloating -r -p cursor
```

# Requirements
```
rust/rustup
sudo
```

# Installation
```
git clone https://github.com/jvc84/hyprfloat
cd hyprfloat
bash install.sh
```









