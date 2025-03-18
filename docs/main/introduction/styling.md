Since astrum depends on `libcosmic`, styling is done within lua and not in another language (css for example)


***
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
