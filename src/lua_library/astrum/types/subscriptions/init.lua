---@meta

---@class Subscriptions
---@field hyprland? HyprlandSubscription
---@field mpris? MprisSubscription
---@field system_tray? SystemTraySubscription
---@field time? table<number, string> # Sends a signal whenever a specified amount (the `key` of the table) is passed
---@field calls? table<string, string> # Sends to a signal when the specified call name (in the index) has been sent
---@field keybinds? table<string, [Modifier, NamedKeys]> # List of signal names (the `key` of the table) that will be sent when pressing down the specified modifiers and a character. The first parameter of the value tuple is `modifiers`, which are seperated by commas (i.e. `"shift,super"` would work when `shift` and `super` are being pressed). The 2nd parameter is a single character or lowercase name for the keybind
---@field notifications? NotificationSubscription
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

---@alias Modifier
---| "ctrl"
---| "shift"
---| "alt"
---| "super"
---| string

---@alias NamedKeys
---| "alt"
---| "backspace"
---| "caps_lock"
---| "ctrl"
---| "enter"
---| "super"
---| "shift"
---| "space"
---| "tab"
---| "delete"
---| "end"
---| "help"
---| "home"
---| "insert"
---| "page_up"
---| "page_down"
---| "arrow_down"
---| "arrow_up"
---| "arrow_left"
---| "arrow_right"
---| "numlock"
---| "escape"
---| "pause"
---| "print_screen"
---| "scroll_lock"
---| "meta"
---| "f1"
---| "f2"
---| "f3"
---| "f4"
---| "f5"
---| "f6"
---| "f7"
---| "f8"
---| "f9"
---| "f10"
---| "f11"
---| "f12"
---| string

---@class (exact) NotificationSubscription
---@field on_notification string # Sends a signal whenever a notification is recieved. The `signal_data` is of type `Notification` and contains the data of the notification.

---@class Notification
---@field app_name string # The optional name of the application sending the notification. Can be blank
---@field replaces_id number # The optional notification ID that this notification replaces. It is reccomended that the server must atomically (i.e. with no visual cues or flicker) replace the given notification with this one, so that applications can modify noitifications while they still are active. A value of 0 means that it will not replace notifications
---@field app_icon string # The optional program icon of the calling application. Can be blank, indicating no icon.
---@field summary string # The summary text briefly describing the notification
---@field body string # The optional detailed body text. Can be blank
---@field actions string[] # Actions are sent over as a list of pairs. Each even element in the list represents the identifier of the action. Each odd element in the list is the localized string that will be displayed to the user
---@field expire_timeout number # The timeout in milliseconds since the display of the notification and at which the notification should automatically close. If it is `-1` it means that there is predefined expire timeout (up to you). If `0` then it never expires.

return subscriptions
