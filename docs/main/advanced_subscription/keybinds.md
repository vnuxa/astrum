# Keybinds

Custom keybinds in astrum are defined through subscriptions\
The index of the keybind susbcription table will be the signal name that it will send to when the specified keys have been pressed.

\
The first parameter of the value table are the modifiers that need to be pressed, seperated by commas.\
The second parameter is a single character or a lowercase name for the keybind (you can see the full list of named keys in the source of the keybind susbcription)

\
An example implementation:

```lua
app:window("notification-thing", {
    keymode = "on_demand",
	subscriptions = {
        keybinds = {
            on_key = { "ctrl,shift", "space"},
            on_key2 = { "ctrl", "d"}
        }
	},
	signals = {
        on_key = function()
            print("ctrl shift and space have been pressed!")
        end,
        on_key2 = function()
            print("ctrl and d have been pressed!")
        end
	},
})
```


>   **Note:**
>   keybinds will not work if the keymode is `none`
