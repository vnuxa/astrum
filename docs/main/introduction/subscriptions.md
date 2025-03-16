Subscriptions are the only way to update your state and view logic after an external event occurs.\
Subscriptions will trigger *every* window's view and signal logic

---
To use a subscription, you must subscribe to a specific service.\
Here is a hyprland workspace changed subscription example:
```lua
local state = {
    -- most services have types
    ---@type HyprlandWorkspaces
    workspaces = {}
}
-- Tells the application to subscribe to the `Hyprland` service, and to specifically use the `workspaces` signal
App:subscribe("hyprland", {"workspaces"})

App:window("workspace_example", {
    view = function()
        local row = Widgets:row({})

        for _, workspace in pairs(state.workspaces) do
            row:push(Widgets:text(tostring(workspace.id)))
        end

        return row
    end,
    signals = {
        -- The `Hyprland` service sends this specific signal name when you subscribe to `workspaces`
        hyprland_workspaces = function(signal_data)
            state.workspaces = signal_data
        end
    }
})

```

Each service has signals which you can subscribe to.\
In this example, the `Hyprland` service sends `hyprland_workspaces` signal when you subscribe to the `workspaces` signal
