# hyprfloat

Hyprfloat is a hyprland plugin, desined to improve your experience with floating windows 

Supports Hyprland `>=42.0`



https://github.com/user-attachments/assets/5505e90a-c387-404c-879b-6cb97b96edca



**Requirements**
```
rust/rustup
sudo
```

**Installation**
```
bash install.sh
```

**Usage**

```
hfresize x y

x - resize by x axis
y - resize by y axis
```


```
hfopen [ARGS] APPLICATION
hftogglefloat [ARGS]

ARGS:
   -p | --position  POSITION  moves window to cursor
    POSITION:
      cursor               moves window to cursor
      opposite             moves window to mirror cursor position
      corner               moves window to the farthest corner from cursor 
      center               moves window to center
  -w | --width WIDTH       resizes window width to WIDTH
  -h |  --height HEIGHT    resizes window hegight to HEIGHT
  -r | --resize            resizes window to 'size' parameter in config
  -t | --tiled             makes window tiled if it is floating
```


```
hfmove DIRECTION

DIRECTION:
  l | left
  r | right
  u | up
  d | down
```




