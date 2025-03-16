# Table of contents

1. [`ButtonModel`](#ButtonModel) 
2. [`CenterboxModel`](#CenterboxModel) 
3. [`ColumnModel`](#ColumnModel) 
4. [`ContainerModel`](#ContainerModel) 
5. [`CustomSignal`](#CustomSignal) 
6. [`IconModel`](#IconModel) 
7. [`ImageModel`](#ImageModel) 
10. [`RowModel`](#RowModel) 
11. [`TextInputModel`](#TextInputModel) 
12. [`TextModel`](#TextModel) 
13. [`Widget`](#Widget) 
14. [`WidthHeightWidget`](#WidthHeightWidget) 

---
# ButtonModel
## Propreties:
>   `child` → `Widget?`
>    >   A widget that will be displayed within the button 

see definitions: [`Widget`](#Widget) 
>   `height` → `("fill"|"shrink"|["fill_portion", number]|["fixed", number])?`
>    >   Sets the height of the widget 

>   `on_press` → `(string|CustomSignal)?`
>    >   Sends a signal whenever the button is pressed. If a string is provided, it will send no data 

see definitions: [`CustomSignal`](#CustomSignal) 
>   `padding` → `(number|[number, number, number, number]|[number, number])?`

>   `width` → `("fill"|"shrink"|["fill_portion", number]|["fixed", number])?`
>    >   Sets the width of the widget 

## Methods:


---
# CenterboxModel
## Propreties:
>   `align_items` → `("center"|"end"|"start")?`

>   `height` → `("fill"|"shrink"|["fill_portion", number]|["fixed", number])?`
>    >   Sets the height of the widget 

>   `left_child` → `Widget?`
>    >   Element to be displayed on the left side of the centerbox 

see definitions: [`Widget`](#Widget) 
>   `middle_child` → `Widget?`
>    >   Element to be displayed in the middle of the centerbox 

see definitions: [`Widget`](#Widget) 
>   `padding` → `(number|[number, number, number, number]|[number, number])?`

>   `right_child` → `Widget?`
>    >   Element to be displayed on the right side of the centerbox 

see definitions: [`Widget`](#Widget) 
>   `spacing` → `number?`
>    >   The spacing of elements in pixels 

>   `width` → `("fill"|"shrink"|["fill_portion", number]|["fixed", number])?`
>    >   Sets the width of the widget 

## Methods:


---
# ColumnModel
## Propreties:
>   `align_x` → `("center"|"left"|"right")?`
>    >   Sets the vertical alignments of the contents of the `column` 

>   `children` → `Widget[]?`
>    >   List of widgets to be rendered within the `column` 

see definitions: [`Widget`](#Widget) 
>   `clip` → `boolean?`
>    >   Sets whether the contents of the `column` should be clipped on overflow 

>   `height` → `("fill"|"shrink"|["fill_portion", number]|["fixed", number])?`
>    >   Sets the height of the widget 

>   `max_width` → `number?`
>    >   Maximum width of the `column` in pixels 

>   `padding` → `(number|[number, number, number, number]|[number, number])?`

>   `spacing` → `number?`
>    >   The spacing between elements in pixels 

>   `width` → `("fill"|"shrink"|["fill_portion", number]|["fixed", number])?`
>    >   Sets the width of the widget 

## Methods:


---
# ContainerModel
## Propreties:
>   `align_x` → `("center"|"left"|"right")?`
>    >   Sets the alignment of content on the horizontal axis 

>   `align_y` → `("bottom"|"center"|"top")?`
>    >   Sets the alignment of content on the vertical axis 

>   `center_x` → `("fill"|"shrink"|["fill_portion", number]|["fixed", number])?`
>    >   Sets the width of the `container` and centers the content horizontally 

>   `center_y` → `("fill"|"shrink"|["fill_portion", number]|["fixed", number])?`
>    >   Sets the height of the `container` and centers the content vertically 

>   `child` → `Widget?`
>    >   Element to be displayed within the `container` 

see definitions: [`Widget`](#Widget) 
>   `height` → `("fill"|"shrink"|["fill_portion", number]|["fixed", number])?`
>    >   Sets the height of the widget 

>   `max_height` → `number?`
>    >   Maximum height of the `container` in pixels 

>   `max_width` → `number?`
>    >   Maximum width of the `container` in pixels 

>   `padding` → `(number|[number, number, number, number]|[number, number])?`

>   `width` → `("fill"|"shrink"|["fill_portion", number]|["fixed", number])?`
>    >   Sets the width of the widget 

## Methods:


---
# CustomSignal
## Propreties:
>   `signal_data` → `table`

>   `signal_name` → `string`

## Methods:


---
# IconModel
## Propreties:
>   `content_fit` → `("contain"|"cover"|"fill"|"none"|"scale_down")?`
>    >   Sets how the content should be fit. 

>   `height` → `("fill"|"shrink"|["fill_portion", number]|["fixed", number])?`
>    >   Sets the height of the widget 

>   `icon_name` → `string?`
>    >   The name of the icon. `icon_name` or `icon_path` is required. 

>   `icon_path` → `string?`
>    >   The path to the icon. `icon_name` or `icon_path` is required. 

>   `size` → `integer?`
>    >   The size of the icon. 

>   `width` → `("fill"|"shrink"|["fill_portion", number]|["fixed", number])?`
>    >   Sets the width of the widget 

## Methods:


---
# ImageModel
## Propreties:
>   `border_radius` → `(number|[number, number, number, number])?`
>    >   The border radius of the image 

>   `content` → `string?`
>    >   A path to an image, this field is required 

>   `content_fit` → `("contain"|"cover"|"fill"|"none"|"scale_down")?`
>    >   Sets how the content should be fit. Defaults to `contain` 

>   `filter_method` → `("linear"|"nearest")?`

>   `height` → `("fill"|"shrink"|["fill_portion", number]|["fixed", number])?`
>    >   Sets the height of the widget 

>   `opacity` → `number?`
>    >   Sets the opacity of an image. It should be in `0.0 - 1.0` range 

>   `rotation` → `(["floating", number]|["solid", number])?`
>    >   Sets the rotation of the image. `floating` - element will float while rotating, layout will be the same prior to rotation (default). `solid` - element will be solid while rotating, layout will be adjusted to fit rotated content 

>   `width` → `("fill"|"shrink"|["fill_portion", number]|["fixed", number])?`
>    >   Sets the width of the widget 

## Methods:


---
# RowModel
## Propreties:
>   `align_y` → `("bottom"|"center"|"top")?`
>    >   Sets the vertical alignments of the contents of the `row` 

>   `children` → `Widget[]?`
>    >   List of widgets to be rendered within the `row` 

see definitions: [`Widget`](#Widget) 
>   `clip` → `boolean?`
>    >   Sets whether the contents of the `row` should be clipped on overflow 

>   `height` → `("fill"|"shrink"|["fill_portion", number]|["fixed", number])?`
>    >   Sets the height of the widget 

>   `padding` → `(number|[number, number, number, number]|[number, number])?`

>   `spacing` → `number?`
>    >   The spacing between elements in pixels 

>   `width` → `("fill"|"shrink"|["fill_portion", number]|["fixed", number])?`
>    >   Sets the width of the widget 

## Methods:


---
# TextInputModel
## Propreties:
>   `always_active` → `boolean?`
>    >   If enabled, makes it behave as if it were always focused 

>   `line_height` → `(["absolute", number]|["relative", number])?`
>    >   Sets the line height of the 

>   `on_input` → `string?`
>    >   Runs a signal when some text is typed in the text input, sends `text` in the signal data which contains the new text. Cannot pass through custom signals 

>   `on_submit` → `(string|CustomSignal)?`
>    >   Sends a custom signal when the text input is focused and the enter key is pressed 

see definitions: [`CustomSignal`](#CustomSignal) 
>   `password` → `boolean?`
>    >   If the text input should be a secure password input 

>   `placeholder` → `string?`
>    >   Placeholder text for the text input 

>   `size` → `number?`
>    >   Sets the text size of the text input 

>   `value` → `string?`
>    >   The text of the text input. Needs an external variable paired with `on_input` in order to change 

>   `width` → `("fill"|"shrink"|["fill_portion", number]|["fixed", number])?`

## Methods:


---
# TextModel
## Propreties:
>   `content` → `string?`
>    >   The text that should be displayed 

>   `font` → `Font?`
>    >   The font of the text 

see definitions: [`Font`](./misc.md#Font) 
>   `height` → `("fill"|"shrink"|["fill_portion", number]|["fixed", number])?`
>    >   Sets the height of the widget 

>   `size` → `number?`
>    >   The font size of the text 

>   `width` → `("fill"|"shrink"|["fill_portion", number]|["fixed", number])?`
>    >   Sets the width of the widget 

## Methods:


---
# Widget
## Propreties:
>   `widget_name` → `string`

## Methods:


---
# WidthHeightWidget
## Propreties:
>   `height` → `("fill"|"shrink"|["fill_portion", number]|["fixed", number])?`
>    >   Sets the height of the widget 

>   `width` → `("fill"|"shrink"|["fill_portion", number]|["fixed", number])?`
>    >   Sets the width of the widget 

## Methods:


---
