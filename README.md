# hyprfloat

Improve your experience with floating windows!

Supports Hyprland `>=42.0`

# hfopen

https://github.com/user-attachments/assets/e70fcc55-a50f-483f-a1ec-5589ac08de68

**Example** `hyprland.conf`:
```
...
bind = Super Shift, Return, exec, hfopen -o -w 600 -h 450 -p cursor kitty
bind = Super Shift, R, exec, hfopen -o -w 700 -h 650 -p center "kitty ranger"
...
```

# hftogglefloating

https://github.com/user-attachments/assets/6816a7fa-ec8d-48fa-9fc5-b21af640e069

**Example** `hyprland.conf`:
```
...
bind = Super Shift, Space, exec, hftogglefloating -p cursor
bind = Super Shift, Space, exec, hftogglefloating -r -p center
...
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

# Exampl

```
hfresize x y

x   resize by x axis
y   resize by y axis
```


```
hfopen [ARGS] APPLICATION
hftogglefloat [ARGS]

ARGS:
   -p | --position  POSITION  moves window to POSITION
       POSITION:
         cursor      moves window to cursor
         opposite    moves window to mirror cursor position
         corner      moves window to the farthest corner from cursor 
         center      moves window to center
         random      moves window to random position on screen

  -w | --width WIDTH       resizes window width to WIDTH
  -h | --height HEIGHT     resizes window hegight to HEIGHT
  -r | --resize            resizes window to 'size' parameter in config
  -t | --tiled             makes window tiled if it is floating
```


```
USAGE:

    hfmovewindow [ARGUMENTS] [DIRECTION]

ARGUMENTS:\\n\n    --help                                - show this message\\n    -c PATH | --config PATH               - define PATH for config\\n    -p PARAMETER | --position PARAMETER   - move/open window according to PARAMETER\\n        PARAMETERS:\\n            cursor      - move/open window at the cursor position\\n            center      - move/open window at the center\\n            close       - move/open window at the closest corner from cursor\\n            far         - move/open window at the farthest corner from cursor\\n            opposite    - move/open window at the mirror of cursor position\\n            random      - move/open window at the random position on screen\\n\nDIRECTIONS:\\n\n    l        - move window left according to config parameters\\n    r        - move window right according to config parameters\\n    u        - move window up according to config parameters\\n    d        - move window down according to config parameters\\n\nDEFAULT CONFIG PA
```




