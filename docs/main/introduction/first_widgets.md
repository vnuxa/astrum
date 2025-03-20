Start by making `~/.config/astrum/config.lua`
```lua
Astrum = require("astrum")
App = Astrum:application()
-- you can make windows with App:window()

return App
```

You will see nothing happening if you run astrum, mainly because its an empty config\
You have to make a window in order for astrum to display something

***

## Making windows

Windows are toplevel containers that hold widgets and signals (More on signals later).
```lua
App:window("window_name", {
 -- window model goes here
})
```
\
Windows have a mandatory ``view`` field, which defines what widgets to display\
The ``view`` field gets ran after signals get processed.
```lua
App:window("window_name", {
    view = function()
         return Astrum.widgets:text("hello world!")
    end
})
```
This will display a window that has a text widget containing ``"hello world!"``

***

### Signals and states

Windows also have an optional `signals` field, which is a dictionary of signal names and logic that will be processed when the signal is fired

```lua
App:window("window_name", {
    view = function()
         return Astrum.widgets:button({
             child = Astrum.widgets:text("press me!")
             on_press = "button_pressed" -- the name of the signal
         })
    end,
    -- the syntax for signals is this
    -- signal_name = function(signal_data) end
    signals = {
        button_pressed = function() -- signal data is unnecessary since we do not send any yet
            print("hello world!")
        end
    }
})
```
This example will make a window, that has a button which when pressed will print ``hello world!``\
\
Signals are also the main way you change the state within astrum\
And view logic is where you define widgets based on state

```lua
local state = {
    widget_text = "false"
}

App:window("window_name", {
    view = function()
         return Astrum.widgets:button({
             child = Astrum.widgets:text(state.widget_text)
             on_press = "button_pressed"
         })
    end,

    signals = {
        button_pressed = function()
            if state.widget_text == "false" then
                state.widget_text = "true"
            else
                state.widget_text = "false"
            end
        end
    }
})
```
This will make a window that has a button with text that toggles between `false` and `true`
