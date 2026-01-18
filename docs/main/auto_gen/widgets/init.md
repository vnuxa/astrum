# Table of contents

1. [`Widgets`](#widgets) 

[`source`](https://github.com/vnuxa/astrum/blob/master/src/lua_library/astrum/types/widgets/init.lua)

---
# Widgets
## Propreties:
## Methods:
`:button(content_or_model, extra_model)` → `Widget`
>    `content_or_model`: `(string|ButtonModel)?`
>    >   The text to be displayed (shorthand) or the model for the button 

>    `extra_model`: `ButtonModel?`
>    >   The params for the shorthand. You do not need this if you provide a model in the first argument 


`:centerbox(model)` → `Widget`
>    `model`: `CenterboxModel?`


`:column(model)` → `table`
>    `model`: `ColumnModel?`


`:container(model)` → `Widget`
>    `model`: `ContainerModel`


`:context_menu(model_or_child, tree)` → `Widget`
>    `model_or_child`: `ContextMenuModel|Widget`
>    >   The model for the context menu or the underlying element for it 

>    `tree`: `TreeWidget[]?`
>    >   The menu tree that will be displayed when the right mouse button has been pressed on the underlying element 


`:icon(icon_name_or_model, extra_model)` → `Widget`
>    `icon_name_or_model`: `(string|IconModel)?`
>    >   The icon name (shorthand) or the model for the icon 

>    `extra_model`: `IconModel?`
>    >   Extra params for the shorthand. You do not need this if you already provided a model in the first argument 


`:image(content_or_model, extra_model)` → `Widget`
>    `content_or_model`: `(string|ImageModel)?`
>    >   The text to be displayed (shorthand) or the model for the image 

>    `extra_model`: `ImageModel?`
>    >   The params for the shorthand. You do not need this if you provide a model in the first argument 


`:mouse_area(model)` → `Widget`
>    `model`: `MouseAreaModel`


`:row(model)` → `table`
>    `model`: `RowModel?`


`:scrollable(model)` → `Widget`
>    `model`: `ScrollableModel`


`:signal(signal_name, signal_data)` → `CustomSignal`
>    `signal_name`: `string`

>    `signal_data`: `table`
>    >   Data to be sent through the signal 


`:space(width, height)` → `Widget`
>    `width`: `("fill"|"shrink"|["fill_portion", number]|["fixed", number])?`
>    >   The width of the empty amount of space. Defaults to `"shrink"` 

>    `height`: `("fill"|"shrink"|["fill_portion", number]|["fixed", number])?`
>    >   The height of the empty emount of space. Defaults to `"shrink"` 


`:stack(model_or_children)` → `Widget`
>    `model_or_children`: `StackModel|Widgets[]`
>    >   The model for the stack or a list of widgets. The first element of this list will determine the intrinsic size of the stack, every other element will be rendered on top; on its own layer 


`:text(content_or_model, extra_model)` → `Widget`
>    `content_or_model`: `(string|TextModel)?`
>    >   The text to be displayed (shorthand) or the model for the text 

>    `extra_model`: `TextModel?`
>    >   Extra params for the shorthand. You do not need this if you provide a model in the first argument 


`:text_input(content_or_model, placeholder, model)` → `Widget`
>    `content_or_model`: `string|TextInputModel`

>    `placeholder`: `string?`

>    `model`: `TextInputModel?`


`:tree(model_or_item, tree)`
>    `model_or_item`: `TreeWidget|Widget`
>    >   The model for the tree or the core item of the tree. 

>    `tree`: `TreeWidget[]?`
>    >   Defines the `tree` field for the model. 




---
