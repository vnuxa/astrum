# Table of contents

1. [`Astrum`](#astrum) 
2. [`AstrumApp`](#astrumapp) 
3. [`WindowModel`](#windowmodel) 

[`source`](https://github.com/vnuxa/astrum/blob/unstable/src/lua_library/astrum/types/init.lua)

---
# Astrum
## Propreties:
## Methods:
`:application()` → `AstrumApp`

`:toggle_window(window_name)`
>    `window_name`: `string`




---
# AstrumApp
## Propreties:
## Methods:
`:window(window_name, window_model)`
>    `window_name`: `string`
>    >   The unique name of the window 

>    `window_model`: `WindowModel`
>    >   The model of the window 




---
# WindowModel
## Propreties:
>   `anchor` → `"bottom"|"left"|"right"|"top"[]?`

>   `exclusive_zone` → `(integer|"ignore")?`
>    >   How much space should the window reserve, set it to `"ignore"` if you want it to ignore other layers 

>   `height` → `number?`
>    >   If provided, manually set the height of the window 

>   `is_popup` → `boolean?`

>   `keymode` → `("exclusive"|"none"|"on_demand")?`

>   `layer` → `("background"|"bottom"|"top")?`

>   `signals` → `({ [string]: fun(signal_data: table) }|{ [string]: fun(signal_data: table) }[])?`
>    >   A dictionary of signal names and their respective logic that will be processed when the signal is called on. If a table value is provided, it will unpack it. If there are multiple signals with the same name, it will get overriden 

>   `subscriptions` → `Subscriptions?`
>    >   Connects to an external processes by sending signals. All of the subscriptions are to be provided in a table 

see definitions: [`Subscriptions`](./subscriptions/init.md#subscriptions) 
>   `view` → `fun():Widget`
>    >   Logic that dictates what widgets for the window to render 

see definitions: [`Widget`](./widgets/models.md#widget) 
## Methods:


---
