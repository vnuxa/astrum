# Table of contents

1. [`Animation`](#animation) 
2. [`Animations`](#animations) 

[`source`](https://github.com/vnuxa/astrum/blob/unstable/src/lua_library/astrum/types/animations/init.lua)

---
# Animation
## Propreties:
>   `animation_id` → `number`
>    >   The animation's ID, used for interracting with lilt animations 

## Methods:
`:animate_table(false_table, true_table)`
>    `false_table`: `table`
>    >   The table to go to when the `animation state` is `false` 

>    `true_table`: `table`
>    >   The table to go to when the `animation state` is `true` 


`:animate_value(false_value, true_value)`
>    `false_value`: `number`
>    >   The value to transition to when the `animation state` is `false` 

>    `true_value`: `number`
>    >   The value to transition to when the `animation state` is `true` 


`:get_state()` → `boolean`

`:play()`

`:toggle(state)`
>    `state`: `boolean?`
>    >   If provided, change to a specific state 




---
# Animations
## Propreties:
## Methods:
`:animate_style(animation, false_style, true_style)`
>    `animation`: `Animation`

>    `false_style`: `string`
>    >   The classname of the requested style to go to when the `animation` is `false`. 

>    `true_style`: `string`
>    >   The classname of the requested style to go to when the `animation` is `true`. 


`:new(starting_value, time, easing, repeat_amount, reverse, delay)` → `Animation`
>    `starting_value`: `boolean|nil`
>    >   The state of the animation, default is `false` 

>    `time`: `number|nil`
>    >   The amount of seconds it takes for the animation to complete. Default is `1` 

>    `easing`: `"ease_in"|"ease_in_back"|"ease_in_bounce"|"ease_in_circ"|"ease_in_cubic"...(+27)`
>    >   The easing style of the animation, default is `"linear"` 

>    `repeat_amount`: `number|nil`
>    >   The amount of times the animation will repeat itself, default is `nil` 

>    `reverse`: `boolean|nil`
>    >   Will the animation reverse itself, default is `false` 

>    `delay`: `number|nil`
>    >   The delay until the animation will start, in seconds. Default is `0` 




---
