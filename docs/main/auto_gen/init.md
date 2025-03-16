# Table of contents

1. [`Astrum`](#Astrum) 
2. [`AstrumApp`](#AstrumApp) 
3. [`WindowModel`](#WindowModel) 

---
# Astrum
## Propreties:
## Methods:
`:application()` → `AstrumApp`



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

>   `is_popup` → `boolean?`

>   `keymode` → `("exclusive"|"none"|"on_demand")?`

>   `layer` → `("background"|"bottom"|"top")?`

>   `signals` → `({ [string]: fun(signal_data: table) }|{ [string]: fun(signal_data: table) }[])?`
>    >   A dictionary of signal names and their respective logic that will be processed when the signal is called on. If a table value is provided, it will unpack it. If there are multiple signals with the same name, it will get overriden 

>   `subscriptions` → `Subscriptions?`
>    >   Connects to an external processes by sending signals. All of the subscriptions are to be provided in a table 

see definitions: [`Subscriptions`](./subscriptions/init.md#Subscriptions) 
>   `view` → `fun():Widget`
>    >   Logic that dictates what widgets for the window to render 

see definitions: [`Widget`](./widgets/models.md#Widget) 
## Methods:


---
