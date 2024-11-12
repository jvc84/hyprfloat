# Hyprfloat

Improve your experience with floating windows!

Supports Hyprland `>=42.0`

 <details> 
  <summary>Demonstration and examples</summary>

# hfopen

https://github.com/user-attachments/assets/e70fcc55-a50f-483f-a1ec-5589ac08de68

`hyprland.conf`:
```
bind = Super Shift, Return, exec, hfopen -o -s 600x450 -p cursor kitty
bind = Super Shift, R, exec, hfopen -o -s 700x650 -p center "kitty ranger"
```

# hftogglefloating

https://github.com/user-attachments/assets/6816a7fa-ec8d-48fa-9fc5-b21af640e069

`hyprland.conf`:
```
bind = Super Shift, Space, exec, hftogglefloating -p center
bind = Super, Space, exec, hftogglefloating -d -p cursor
```

# hfresizeactive

`hyprland.conf`:
```
bind = Super Alt, Left , exec, hfresizeactive  -100   0    
bind = Super Alt, Down , exec, hfresizeactive   0     100     
bind = Super Alt, Up   , exec, hfresizeactive   0    -100    
bind = Super Alt, Right, exec, hfresizeactive   100   0     

```

# hfmovewindow

`hyprland.conf`:
```
bind = Super SHIFT, Left , exec, hfmovewindow l    
bind = Super SHIFT, Down , exec, hfmovewindow d   
bind = Super SHIFT, Up   , exec, hfmovewindow u   
bind = Super SHIFT, Right, exec, hfmovewindow r   
```

</details>


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









