<div align="center">
  <h1> 
    <img src="https://cyber.dabamos.de/88x31/blink-0.gif" width="88" height="31"/>   
    HYPRFLOAT    
    <img src="https://cyber.dabamos.de/88x31/blink-0.gif" width="88" height="31"/> 
  </h1>
</div>

<h2> Improve your experience with floating windows! </h2>

Supports `Hyprland >= 42.0`

Hyprfloat is a project, presented by 4 console apps based on common library: `hfopen`, `hftogglefloating`, `hfresizeactive`, `hfmovewindow`.
This project is designed to simplify control of floating windows in Hyprland with keyboard and customize their behaviour using config and console arguments

# Demonstration


<details> 
  <summary>hfopen</summary>

# hfopen



https://github.com/user-attachments/assets/df9a8e61-21b2-4da5-9ee4-b65b056d487f



## `hyprland.conf`:
```
bind = Super Shift, Return, exec, hfopen -s 600 450 -p cursor kitty
bind = Super Shift, R, exec, hfopen -s 700 650 -p center "nautilus --new-window"
bind = Super Shift, F, exec, hfopen -d -p cursor firefox
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

https://github.com/user-attachments/assets/3d1471b7-59eb-45be-81c7-cb939ed753ba

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


# Information

You can get this information by using flag `--help` or `-h` with any binary

Default config path: `$HOME/.config/hyprfloat/hf.toml`


<details> 
  <summary>hfopen</summary>

# hfopen
```
Usage: hfopen [OPTIONS] <EXECUTABLE>

Arguments:
<EXECUTABLE>  Program to run (Example: "nautilus --new-window")

Options:
-f, --force                   Do not detect padding, even if 'detect_padding' option in config equals 'true'
-d, --default-size            Resize window according to config parameter 'default_size'
-o, --origin-size             Open small window and then resize it
-t, --tiled                   Open window floating, then tile
-s, --size <SIZE_X> <SIZE_Y>  Set window size by x-axis to <SIZE_X>, by y-axis to <SIZE_Y>
-a, --at <AT_X> <AT_Y>        Set window open position by x-axis to <POS_X>, by y-axis to <POS_Y>
-p, --position <POSITION>     Open window according to <POSITION> value [possible values: l, left, r, right, t, top, b, bottom, tl, top-left, tr, top-right, bl, bottom-left, br, bottom-right, cursor, center, random, far, close, opposite, any]
-c, --config <CONFIG>         Path to config file [default: /home/adex/.config/hyprfloat/hf.toml]
-h, --help                    Print help
-V, --version                 Print version
```

</details>

<details> 
  <summary>hftogglefloating</summary>

# hftogglefloating

```
Usage: hftogglefloating [OPTIONS]

Options:
  -f, --force                   Do not detect padding, even if 'detect_padding' option in config equals 'true'
  -d, --default-size            Resize window according to config parameter 'default_size'
  -s, --size <SIZE_X> <SIZE_Y>  Set window size by x axis to <SIZE_X>, by y axis to <SIZE_Y>
  -a, --at <AT_X> <AT_Y>        Set window open position by x axis to <POS_X>, by y axis to <POS_Y>
  -p, --position <POSITION>     Open window according to <POSITION> value [possible values: l, left, r, right, t, top, b, bottom, tl, top-left, tr, top-right, bl, bottom-left, br, bottom-right, cursor, center, random, far, close, opposite, any]
  -c, --config <CONFIG>         Path to config file [default: /home/adex/.config/hyprfloat/hf.toml]
  -h, --help                    Print help
  -V, --version                 Print version
                                                
```
</details>

<details> 
  <summary>hfresizeactive</summary>
# hfresizeactive

```
Usage: hfresizeactive [OPTIONS] <RESIZE_X> <RESIZE_Y>

Arguments:
  <RESIZE_X>  resize window by x-axis on <RESIZE_X> pixels according to config parameters
  <RESIZE_Y>  resize window by y-axis on <RESIZE_Y> pixels according to config parameters

Options:
  -f, --force            Do not detect padding, even if 'detect_padding' option in config equals 'true'
  -n, --no-invert        Do not invert resize in stick mode, even if 'invert_resize_in_stick_mode' option in config equals 'true'
  -e, --exact            Set size of floating window exactly <RESIZE_X> pixels on x-axis, <RESIZE_Y> pixels on y-axis
  -c, --config <CONFIG>  Path to config file [default: /home/adex/.config/hyprfloat/hf.toml]
  -h, --help             Print help
  -V, --version          Print version
```
</details>

<details> 
  <summary>hfmovewindow</summary>

# hfmovewindow

```
Usage: hfmovewindow [OPTIONS] [DIRECTION]

Arguments:
  [DIRECTION]  Direction to move window to [possible values: l, r, u, d]

Options:
  -f, --force                Do not detect padding, even if 'detect_padding' option in config equals 'true'
  -p, --position <POSITION>  Open window according to <POSITION> value [possible values: l, left, r, right, t, top, b, bottom, tl, top-left, tr, top-right, bl, bottom-left, br, bottom-right, cursor, center, random, far, close, opposite, any]
  -c, --config <CONFIG>      Path to config file [default: /home/adex/.config/hyprfloat/hf.toml]
  -h, --help                 Print help
  -V, --version              Print version
 ```

</details>
