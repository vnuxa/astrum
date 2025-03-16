# Table of contents

1. [`Widgets`](#Widgets) 

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


`:row(model)` → `table`
>    `model`: `RowModel?`


`:signal(signal_name, signal_data)` → `CustomSignal`
>    `signal_name`: `string`

>    `signal_data`: `table`
>    >   Data to be sent through the signal 


`:text(content_or_model, extra_model)` → `Widget`
>    `content_or_model`: `(string|TextModel)?`
>    >   The text to be displayed (shorthand) or the model for the text 

>    `extra_model`: `TextModel?`
>    >   Extra params for the shorthand. You do not need this if you provide a model in the first argument 


`:text_input(content_or_model, placeholder, model)` → `Widget`
>    `content_or_model`: `string|TextInputModel`

>    `placeholder`: `string?`

>    `model`: `TextInputModel?`




---
