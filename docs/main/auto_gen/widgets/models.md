# Table of contents

1. [`ButtonModel`](#buttonmodel) 
2. [`CenterboxModel`](#centerboxmodel) 
3. [`ColumnModel`](#columnmodel) 
4. [`ContainerModel`](#containermodel) 
5. [`CustomSignal`](#customsignal) 
6. [`IconModel`](#iconmodel) 
7. [`ImageModel`](#imagemodel) 
10. [`MouseAreaModel`](#mouseareamodel) 
11. [`OnScrollSignal`](#onscrollsignal) 
12. [`RowModel`](#rowmodel) 
13. [`ScrollableModel`](#scrollablemodel) 
14. [`TextInputModel`](#textinputmodel) 
15. [`TextModel`](#textmodel) 
16. [`Widget`](#widget) 
17. [`WidthHeightWidget`](#widthheightwidget) 

[`source`](https://github.com/vnuxa/astrum/blob/master/src/lua_library/astrum/types/widgets/models.lua)

---
# ButtonModel
## Propreties:
>   `child` → `Widget?`
>    >   A widget that will be displayed within the button 

see definitions: [`Widget`](#widget) 
>   `height` → `("fill"|"shrink"|["fill_portion", number]|["fixed", number])?`
>    >   Sets the height of the widget 

>   `on_press` → `(string|CustomSignal)?`
>    >   Sends a signal whenever the button is pressed. If a string is provided, it will send no data 

see definitions: [`CustomSignal`](#customsignal) 
>   `padding` → `(number|[number, number, number, number]|[number, number])?`

>   `style` → `ButtonAppearance?`
>    >   Sets the appearance of the button 

see definitions: [`ButtonAppearance`](../style/init.md#buttonappearance) 
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

see definitions: [`Widget`](#widget) 
>   `middle_child` → `Widget?`
>    >   Element to be displayed in the middle of the centerbox 

see definitions: [`Widget`](#widget) 
>   `padding` → `(number|[number, number, number, number]|[number, number])?`

>   `right_child` → `Widget?`
>    >   Element to be displayed on the right side of the centerbox 

see definitions: [`Widget`](#widget) 
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

see definitions: [`Widget`](#widget) 
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

see definitions: [`Widget`](#widget) 
>   `height` → `("fill"|"shrink"|["fill_portion", number]|["fixed", number])?`
>    >   Sets the height of the widget 

>   `max_height` → `number?`
>    >   Maximum height of the `container` in pixels 

>   `max_width` → `number?`
>    >   Maximum width of the `container` in pixels 

>   `padding` → `(number|[number, number, number, number]|[number, number])?`

>   `style` → `ContainerAppearance?`
>    >   Sets the appearance of the container 

see definitions: [`ContainerAppearance`](../style/init.md#containerappearance) 
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
# MouseAreaModel
## Propreties:
>   `child` → `Widget`
>    >   Element that determines the size of the mouse area 

see definitions: [`Widget`](#widget) 
>   `on_double_click` → `CustomSignal?`
>    >   Sends a signal when the left mouse button has been pressed twice over a specified area. 

see definitions: [`CustomSignal`](#customsignal) 
>   `on_drag` → `CustomSignal?`
>    >   Sends a signal when a drag has been initiated over a specified area. 

see definitions: [`CustomSignal`](#customsignal) 
>   `on_enter` → `CustomSignal?`
>    >   Sends a signal when the mouse has entered a specified area 

see definitions: [`CustomSignal`](#customsignal) 
>   `on_exit` → `CustomSignal?`
>    >   Sends a signal when the mouse has left a specified area 

see definitions: [`CustomSignal`](#customsignal) 
>   `on_middle_press` → `CustomSignal?`
>    >   Sends a signal when the middle mouse button has been pressed over a specified area. 

see definitions: [`CustomSignal`](#customsignal) 
>   `on_press` → `CustomSignal?`
>    >   Sends a signal when the left mouse button has been pressed over a specified area. 

see definitions: [`CustomSignal`](#customsignal) 
>   `on_release` → `CustomSignal?`
>    >   Sends a signal when the left mouse button has been released over a specified area. 

see definitions: [`CustomSignal`](#customsignal) 
>   `on_scroll` → `string?`
>    >   Sends to a specified signal name, sends `direction` field in a table that can be either `up` or `down` (e.g. OnScrollSignal) 

## Methods:


---
# OnScrollSignal
## Propreties:
>   `direction` → `"down"|"up"`

## Methods:


---
# RowModel
## Propreties:
>   `align_y` → `("bottom"|"center"|"top")?`
>    >   Sets the vertical alignments of the contents of the `row` 

>   `children` → `Widget[]?`
>    >   List of widgets to be rendered within the `row` 

see definitions: [`Widget`](#widget) 
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
# ScrollableModel
## Propreties:
>   `child` → `Widget?`
>    >   Infinite amount of content to be displayed within the scrollable 

see definitions: [`Widget`](#widget) 
>   `direction` → `(["both", { vertical: ScrollablePropreties, horizontal: ScrollablePropreties }]|["horizontal", ScrollablePropreties]|["vertical", ScrollablePropreties])?`
>    >   The direction where the content will be scrolled 

see definitions: [`ScrollablePropreties`](./misc.md#scrollablepropreties) 
>   `height` → `("fill"|"shrink"|["fill_portion", number]|["fixed", number])?`
>    >   Sets the height of the widget 

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

see definitions: [`CustomSignal`](#customsignal) 
>   `password` → `boolean?`
>    >   If the text input should be a secure password input 

>   `placeholder` → `string?`
>    >   Placeholder text for the text input 

>   `size` → `number?`
>    >   Sets the text size of the text input 

>   `style` → `TextInputAppearance?`
>    >   Sets the appearance of the text input 

see definitions: [`TextInputAppearance`](../style/init.md#textinputappearance) 
>   `value` → `string?`
>    >   The text of the text input. Needs an external variable paired with `on_input` in order to change 

>   `width` → `("fill"|"shrink"|["fill_portion", number]|["fixed", number])?`

## Methods:


---
# TextModel
## Propreties:
>   `align_x` → `("center"|"left"|"right")?`
>    >   Sets the horizontal alignment of the text 

>   `align_y` → `("bottom"|"center"|"top")?`
>    >   Sets the vertical alignment of the text 

>   `content` → `string?`
>    >   The text that should be displayed 

>   `font` → `Font?`
>    >   The font of the text 

see definitions: [`Font`](./misc.md#font) 
>   `height` → `("fill"|"shrink"|["fill_portion", number]|["fixed", number])?`
>    >   Sets the height of the text 

>   `line_height` → `number?`
>    >   Sets the line height in pixels 

>   `size` → `number?`
>    >   The font size of the text 

>   `style` → `TextAppearance?`
>    >   Sets the appearance of the text 

see definitions: [`TextAppearance`](../style/init.md#textappearance) 
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
