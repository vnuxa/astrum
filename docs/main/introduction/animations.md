Animations are supported within astrum. Animations can be used to transition from 1 numeric value to another.


---

To use animations within astrum you must first create it using the `astrum.animations:new()` function, like so
```lua
local example_data= {
    Animations:new(false, 0.35, "ease_out_expo")
}
```
\
Animations have an inner state which determines which value to use or transition to if the value has been changed
\
To start playing an animation you must toggle its state, which can be done with either the `toggle` function if you want it to be set to a specific state or the `:play` if you just want it toggle between being true/false
```lua
local example_data= {
    animation = Animations:new(false, 0.35, "ease_out_expo")
}

App:window("window_name", {
    view = function()
         return Astrum.widgets:row({
            spacing = example_data.animation:animate_value(0, 50),
            children = {
                Astrum.widgets:button("toggle animation", { on_press = "toggle_animation" }),
                Astrum.widgets:text("text 1"),
                Astrum.widgets:text("text 2"),
            }
         })
    end
    signals = {
        toggle_animation = function()
            example_data.animation:play()
        end,
    }
})
```

This will produce a row with a button and some text, when the button is pressed it will toggle an animation that will change the spacing between the elements within it.\
You can also use animations to transition between styles easily using the `:animate_style()` function in the same manor as the example above
