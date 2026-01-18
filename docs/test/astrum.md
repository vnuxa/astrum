# Documentation

astrum is a widgeting system that depends on [libcosmic](https://github.com/pop-os/libcosmic)
it can run on any wayland compositor that supports the `layer-shell` protocol.

astrum is made in rust and is configured in lua
i reccomend using the [lua language server](https://github.com/LuaLS/lua-language-server) as that is what i will depend on for types
and i reccomend getting familiar with the annotation syntax as that is what i will show in documentation


## Installation

since this project is still a work in progress,
you have to build it from source.

---

if you use nix you can add this as a nix flake input
```nix
inputs = {
    astrum.url = "github:vnuxa/astrum"
}
```
and somewhere in your package list add the following
```nix
    astrum.defaultPackage.${pkgs.system}
```


---
I also recommend having the [lua language server](https://github.com/LuaLS/lua-language-server) installed in your text editor, since it contains astrum types and descriptions

## Styling basics

Since astrum depends on `libcosmic`, styling is done within lua and not in another language (css for example)


## Stylesheets

Although the styling is done within lua, there still are stylesheets
```lua
Astrum.style:add_style("style_name", {
-- style body goes here
})
```
Each widget has its own styling model, though if a widget has multiple styling states (for example buttons `on_pressed` and `on_hovered`) there is a `default` state as well, which overrides the base defaults, if not already specified

You can use stylesheets within widgets via the `get_style` function
```lua
Astrum.widgets:text({
    content = "hello world!",
    style = Astrum.style:get_style("style_name")
})
```


## Subscription basics


Subscriptions are the only way to update your state and view logic after an external event occurs.\
Subscriptions will trigger *every* window's view and signal logic (might change in the future, depending on libcosmic)

---
To use a subscription, you must subscribe to a specific service.\
Here is a hyprland workspace changed subscription example:
```lua
local state = {
    -- most services have types
    ---@type HyprlandWorkspaces
    workspaces = {}
}

App:window("workspace_example", {
    view = function()
        local row = Widgets:row({})

        for _, workspace in pairs(state.workspaces) do
            row:push(Widgets:text(tostring(workspace.id)))
        end

        return row
    end,
    -- Tells the application to subscribe to the `Hyprland` service
    subscriptions = {
        hyprland = {
            ---Specifies that whenever there is a workspace change, send the following signal that goes by the name of `on_workspace`
            workspaces = "on_workspace"
        }
    }
    signals = {
        on_workspace = function(signal_data)
            -- Signal data is of type `HyprlandWorkspaces`
            state.workspaces = signal_data
        end
    }
})

```


# Advanced subscriptions

These subscriptions do not have predefined types that make it easy to understand on how to use it, which is why these will have to be further explained in the documentation


## Calls

Calls is a powerful subscription that allows external proccesses to interact with astrum by sending a call signal.\
Each key in the calls subscription table will be the `call_signal` and each value will be the name of the signal that will fire when it recieved the call signal

```lua
local called_times = 0

app:window("notification-thing", {
    view = function()
        return widgets:text("recieved call `mycall` ".. called_times.. " times")
    end,
    subscriptions = {
        calls = {
            mycall = "on_call"
        }
    },
    signals = {
        on_call = function(signal_data)
            called_times = called_times + 1
        end
    },
})
```
This will display a widget that shows the amount of times that it has recieved the `mycall` call signal\
To send this call signal, add `--call` to the options of astrum (i.e. `astrum --call mycall`)

You can also send data through the call by adding a `:` after the call signal name, though note that it has to be in valid lua table

```lua
local data = ""

app:window("notification-thing", {
    view = function()
        return widgets:text("call `mycall` has sent data: ".. data)
    end,
    subscriptions = {
        calls = {
            mycall = "on_call"
        }
    },
    signals = {
        on_call = function(signal_data)
            data = signal_data.data
        end
    },
})
```

and executing the resulting command

```bash
astrum --call 'mycall:{data="test"}'
```

will result in the window text displaying the following
```
    call `mycall` has send data: test
```


## Keybinds


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


::: {.note}

keybinds will not work if the keymode is `none`

:::

