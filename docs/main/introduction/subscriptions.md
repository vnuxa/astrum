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
