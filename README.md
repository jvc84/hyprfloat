<div align="center">
  <h1> 
    <img src="https://cyber.dabamos.de/88x31/blink-0.gif" width="88" height="31"/>   
    HYPRFLOAT    
    <img src="https://cyber.dabamos.de/88x31/blink-0.gif" width="88" height="31"/> 
  </h1>
</div>
  
<h2> Improve your experience with floating windows! </h2>

Supports `Hyprland >= 42.0`

Hyprfloat is a project, presented by 4 binaries, based on common library: `hfopen`, `hftogglefloating`, `hfresizeactive`, `hfmovewindow`. 
This project is designed to simplify control of floating windows with keyboard and customize their behaviour using config and console arguments

# Information
You can also get this information by using flags `--help` or `-h` with any binary 

<details> 
  <summary>Show</summary>
  
  Default config path:  `$HOME/.config/hyprfloat/hf.toml`
  
  # hfopen
  USAGE: `hfopen [ARGUMENTS] "EXECUTABLE"`

ARGUMENTS:
```
    -h, --help                  - show this message
    -t, --tiled                 - open window tiled
    -o, --origin-size           - let program open window with specific size and then resize it.
        Recommended when size is predefined via config or console arguments
    -d, --default-size          - resize window according to config parameter `default_size`
    -c, --config PATH           - define PATH for config
    -s, --size SIZE_XxSIZE_Y    - set window size by x axis to SIZE_X, by y axis to SIZE_Y
    -m, --move POS_XxPOS_Y      - set window open position by x axis
 to POS_X, by y axis to POS_Y
    -p, --position PARAMETER    - open window according to PARAMETER
        PARAMETERS:
            l, left              to the left center position
            r, right             to the right center position
            t, top               to the top center position
            b, bottom            to the bottom center position
            tl, top-left         to the top-left corner
            tr, top-right        to the top-right corner
            bl, bottom-left      to the bottom-left corner
            br, bottom-right     to the bottom-right corner
            cursor               to the cursor position
            center               to the center
            close                to the closest corner from cursor
            far                  to the farthest corner from cursor
            opposite             to the mirror of cursor position
            random               to the random position on screen
 ```
 
</details>

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


```bash
git clone https://github.com/jvc84/hyprfloat
cd hyprfloat
bash install.sh
```









