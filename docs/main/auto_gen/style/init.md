# Table of contents

1. [`ButtonAppearance`](#buttonappearance) 
2. [`ButtonStyleSheet`](#buttonstylesheet) 
3. [`ContainerAppearance`](#containerappearance) 
4. [`TextAppearance`](#textappearance) 
5. [`TextInputAppearance`](#textinputappearance) 
6. [`TextInputStyleSheet`](#textinputstylesheet) 

[`source`](https://github.com/vnuxa/astrum/blob/unstable/src/lua_library/astrum/types/style/init.lua)

---
# ButtonAppearance
## Propreties:
>   `background` → `rgba?`
>    >   The background of the button 

see definitions: [`rgba`](./misc.md#rgba) [`rgb`](./misc.md#rgb) 
>   `border_color` → `rgba?`
>    >   The color of the button border 

see definitions: [`rgba`](./misc.md#rgba) [`rgb`](./misc.md#rgb) 
>   `border_radius` → `(number|[number, number, number, number])?`
>    >   Sets the border radius of the button 

>   `border_width` → `number?`
>    >   Sets the width of the border within the button 

>   `icon_color` → `rgba?`
>    >   The icon color of the button 

see definitions: [`rgba`](./misc.md#rgba) [`rgb`](./misc.md#rgb) 
>   `outline_color` → `rgba?`
>    >   Sets the color of the outline 

see definitions: [`rgba`](./misc.md#rgba) [`rgb`](./misc.md#rgb) 
>   `outline_width` → `number?`
>    >   An outline placed around the button 

>   `overlay` → `rgba?`
>    >   The background overlay of the button 

see definitions: [`rgba`](./misc.md#rgba) [`rgb`](./misc.md#rgb) 
>   `shadow_offset` → `Vector?`
>    >   The amount of shadow offset to be applied on the button 

see definitions: [`Vector`](./misc.md#vector) 
>   `text_color` → `rgba?`
>    >   The color of the text 

see definitions: [`rgba`](./misc.md#rgba) [`rgb`](./misc.md#rgb) 
## Methods:


---
# ButtonStyleSheet
## Propreties:
>   `active` → `ButtonAppearance?`
>    >   The appearance of the button when it is active 

see definitions: [`ButtonAppearance`](#buttonappearance) 
>   `default` → `ButtonAppearance`
>    >   The default appearance for the button, this default style will fill in the other styles if they are not provided 

see definitions: [`ButtonAppearance`](#buttonappearance) 
>   `disabled` → `ButtonAppearance?`
>    >   The appearance of the button when it is disabled 

see definitions: [`ButtonAppearance`](#buttonappearance) 
>   `hovered` → `ButtonAppearance?`
>    >   The appearance of the button when it is hovered 

see definitions: [`ButtonAppearance`](#buttonappearance) 
>   `pressed` → `ButtonAppearance?`
>    >   The appearance of the button when it is pressed 

see definitions: [`ButtonAppearance`](#buttonappearance) 
## Methods:


---
# ContainerAppearance
## Propreties:
>   `background` → `rgba?`
>    >   The background of the container 

see definitions: [`rgba`](./misc.md#rgba) [`rgb`](./misc.md#rgb) 
>   `border` → `Border?`
>    >   The border of the container 

see definitions: [`Border`](./misc.md#border) 
>   `icon_color` → `rgba?`
>    >   The color of icons within the container 

see definitions: [`rgba`](./misc.md#rgba) [`rgb`](./misc.md#rgb) 
>   `shadow` → `Shadow?`

see definitions: [`Shadow`](./misc.md#shadow) 
>   `text_color` → `rgba?`
>    >   The color of the text 

see definitions: [`rgba`](./misc.md#rgba) [`rgb`](./misc.md#rgb) 
## Methods:


---
# TextAppearance
## Propreties:
>   `text_color` → `rgba?`
>    >   The color of the text 

see definitions: [`rgba`](./misc.md#rgba) [`rgb`](./misc.md#rgb) 
## Methods:


---
# TextInputAppearance
## Propreties:
>   `background` → `rgba?`
>    >   The color of the background 

see definitions: [`rgba`](./misc.md#rgba) [`rgb`](./misc.md#rgb) 
>   `border_color` → `rgba?`
>    >   The color of the border 

see definitions: [`rgba`](./misc.md#rgba) [`rgb`](./misc.md#rgb) 
>   `border_radius` → `number?`
>    >   The border radius 

>   `border_width` → `number?`
>    >   The border width 

>   `placeholder_color` → `rgba?`
>    >   The color of the placeholder text within the text input 

see definitions: [`rgba`](./misc.md#rgba) [`rgb`](./misc.md#rgb) 
>   `selected_fill` → `rgba?`
>    >   The color of the selected text background within the text input 

see definitions: [`rgba`](./misc.md#rgba) [`rgb`](./misc.md#rgb) 
>   `selected_text_color` → `rgba?`
>    >   The color of selected text within the text input 

see definitions: [`rgba`](./misc.md#rgba) [`rgb`](./misc.md#rgb) 
>   `text_color` → `rgba?`
>    >   The color of the text within the text input 

see definitions: [`rgba`](./misc.md#rgba) [`rgb`](./misc.md#rgb) 
## Methods:


---
# TextInputStyleSheet
## Propreties:
>   `active` → `ButtonAppearance?`
>    >   The appearance of the text input when it is active 

see definitions: [`ButtonAppearance`](#buttonappearance) 
>   `default` → `ButtonAppearance`
>    >   The default appearance of the text input, this default style will fill in the other styles if they are not provided 

see definitions: [`ButtonAppearance`](#buttonappearance) 
>   `disabled` → `ButtonAppearance?`
>    >   The appearance of the text input when it is disabled 

see definitions: [`ButtonAppearance`](#buttonappearance) 
>   `error` → `ButtonAppearance?`
>    >   The appearance of the text input when it is errored 

see definitions: [`ButtonAppearance`](#buttonappearance) 
>   `focused` → `ButtonAppearance?`
>    >   The appearance of the text input when it is hovered 

see definitions: [`ButtonAppearance`](#buttonappearance) 
## Methods:


---
