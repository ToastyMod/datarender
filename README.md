# datarender
a visualizer that reads bytes as vertex data.

![](https://i.gyazo.com/6f82cbea09912adce681957ec7204508.png)

# How it works
The selected file is read in groups of 3 as float values (a vector3 of floats)
those are used as 3D vectors which are popped into a vertex buffer and rendered.

# Why?
My young naive 14-year old self made this program in Processing (Java) with the intention of reading 3D model data from game ROMs.
I didn't do a whole lot of research so I didn't know if reading 3 float values at a time would actually work but nonetheless it produced some interesting results. 
Try it on some of your own "legally downloaded" ROMs and see what cool patterns arise.

# Usage
Select any ol' file, turn some knobs, mess wid it.
