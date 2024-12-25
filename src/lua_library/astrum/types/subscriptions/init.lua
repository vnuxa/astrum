---@meta

---@class Subscriptions
---@field hyprland? HyprlandSubscription
---@field mpris? MprisSubscription
---@field system_tray? SystemTraySubscription
---@field time? table<number, string> # Sends a signal whenever a specified amount (the `key` of the table) is passed
local subscriptions = {}

---@class (exact) HyprlandSubscription
---@field workspaces? string # Sends an signal whenever hyprland workspaces change (e.g. workspace is added, workspace is moved, workspace is destroyed). All of the workspaces are provided
---@field clients? string # Sends a signal whenever a hyprland client (window) was changed (for example: window closed, window opened, window moved). All of the clients ordered by their workspace is provided
---@field active_client? string # Sends a signal whenever the active hyprland client (window) was changed (e.g. active window focus has changed). Outputs a singular `HyprlandClient` or `nil` if there is no focus

---@class (exact) MprisSubscription
---@field playing? string # Sends a signal whenever the player started playing
---@field paused? string # Sends a signal whenever the player was paused
---@field stopped? string # Sends a signal whenever the player was stopped
---@field volume_changed? string # Sends a signal whenever the player's volume was changed. New volume is provided
---@field looping_changed? string # Sends a signal whenever the player's loop status was changed. New loop status is provided
---@field shuffle_toggled? string # Sends a signal whenever the player's shuffle status was changed. New shuffle status is provided
---@field track_changed? string # Sends a signal whenever the player's track has changed. New metadata is provided

---@class (exact) SystemTraySubscription
---@field update? string # Sends a signal every time items in the tray get updated

---@alias TrayCategory
---| '"application_status"'
---| '"communications"'
---| '"system_services"'
---| '"hardware"'

---@alias TrayStatus
---| '"active"'
---| '"passive"'
---| '"needs_attention"'
---| '"unknown"'

---@class (exact) SystemTrayItem
---@field id string # A name that is unique for this application
---@field category TrayCategory # The category for this item
---@field status TrayStatus # Describes the status for this item or of the associated application
---@field title? string # A name that describes the application, can be more descriptive than `id`, but it is also not nesscessary.
---@field icon_name? string # The name of the icon that should visualise the tray item
---@field icon_pixmap? string # The id of the pixmap to use.

return subscriptions
