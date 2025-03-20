# Table of contents

1. [`Border`](#border) 
2. [`Shadow`](#shadow) 
3. [`Vector`](#vector) 
4. [`rgb`](#rgb) 
5. [`rgba`](#rgba) 

[`source`](https://github.com/vnuxa/astrum/blob/unstable/src/lua_library/astrum/types/style/misc.lua)

---
# Border
## Propreties:
>   `color` → `rgba?`
>    >   The color of the border 

see definitions: [`rgba`](#rgba) [`rgb`](#rgb) 
>   `radius` → `(number|[number, number, number, number])?`
>    >   The radius of the border 

>   `width` → `number?`
>    >   The width of the border 

## Methods:


---
# Shadow
## Propreties:
>   `blur_radius` → `number?`
>    >   The blur radius of the shadow 

>   `color` → `rgba?`
>    >   The color of the shadow 

see definitions: [`rgba`](#rgba) [`rgb`](#rgb) 
>   `offset` → `Vector?`
>    >   The offset of the shadow 

see definitions: [`Vector`](#vector) 
## Methods:


---
# Vector
## Propreties:
>   `x` → `number`
>    >   The X component of the vector 

>   `y` → `number`
>    >   the Y component of the vector 

## Methods:


---
# rgb
## Propreties:
>   `blue` → `number`
>    >   Value ranging from 0-255 

>   `green` → `number`
>    >   Value ranging from 0-255 

>   `red` → `number`
>    >   Value ranging from 0-255 

## Methods:


---
# rgba
## Propreties:
>   `alpha` → `number`
>    >   Value ranging from 0.0-1.0 

>   `blue` → `number`
>    >   Value ranging from 0-255 

>   `green` → `number`
>    >   Value ranging from 0-255 

>   `red` → `number`
>    >   Value ranging from 0-255 

## Methods:


---
