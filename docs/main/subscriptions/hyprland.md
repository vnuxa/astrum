## Signals

### `workspaces`
Sends a `hyprland_workspaces` signal whenever a workspace gets changed in `Hyprland`\
with the following `signal_data`:
```lua
signal_data: HyprlandWorkspaces

---@alias HyprlandWorkspaces HyprlandWorkspace[]

---@class (exact) HyprlandWorkspace
---@field id number
---@field active boolean

```

---

## Methods

### `services.hyprland:set_workspace(workspace)`
Sets the active workspace to the one specified within hyprland
```lua
---@param workspace number
```

### `services.hyprland:get_active_workspace()`
Gets the current active workspace id
```lua
---@return integer
```

[see testing](main.md#Testing)
