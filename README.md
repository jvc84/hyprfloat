# Hyprfloat

Improve your experience with floating windows!

Supports Hyprland `>=42.0`

# Demonstration

<details> 
  <summary>hfopen</summary>

# hfopen



https://github.com/user-attachments/assets/df9a8e61-21b2-4da5-9ee4-b65b056d487f



## `hyprland.conf`:
```
bind = Super Shift, Return, exec, hfopen -o -s 600x450 -p cursor kitty
bind = Super Shift, R, exec, hfopen -o -s 700x650 -p center "kitty ranger"
```
</details>

<details> 
  <summary>hftogglefloating</summary>
  
# hftogglefloating



https://github.com/user-attachments/assets/ee18e752-b0b0-4248-b1af-e6c0b4ae8098



## `hyprland.conf`:
```
bind = Super Shift, Space, exec, hftogglefloating -p center
bind = Super, Space, exec, hftogglefloating -d -p cursor
```
</details>

<details> 
  <summary>hfresizeactive</summary>
  
# hfresizeactive



https://github.com/user-attachments/assets/554d927b-b9d3-4c7a-bb47-773bae5ae722



## `hyprland.conf`:
```
bind = Super Alt, Left , exec, hfresizeactive  -100   0    
bind = Super Alt, Down , exec, hfresizeactive   0     100     
bind = Super Alt, Up   , exec, hfresizeactive   0    -100    
bind = Super Alt, Right, exec, hfresizeactive   100   0     

```
</details>

<details> 
  <summary>hfmovewindow</summary>
  
# hfmovewindow



https://github.com/user-attachments/assets/aa839f2b-d0c5-4156-97d8-ae394889c62e



## `hyprland.conf`:
```
bind = Super Shift, Left , exec, hfmovewindow l    
bind = Super Shift, Down , exec, hfmovewindow d   
bind = Super Shift, Up   , exec, hfmovewindow u   
bind = Super Shift, Right, exec, hfmovewindow r   
```
```
bind = Super Shift, X, exec, hfmovewindow -p center
bind = Super Shift, C, exec, hfmovewindow -p cursor
bind = Super Shift, Z, exec, hfmovewindow -p far 
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









