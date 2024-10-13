---@meta

---@class Services
local services = {}

-- hyprland service

---@class HyprlandService
services.hyprland = {}

---@class (exact) HyprlandWorkspace
---@field id number
---@field active boolean

---@alias HyprlandWorkspaces HyprlandWorkspace[]

---Sets the active workspace to the one specified within hyprland
---@param workspace number
function services.hyprland:set_workspace(workspace) end

---Gets the current active workspace
---@return integer
function services.hyprland:get_active_workspace() end

-- niri service

---@class NiriService
services.niri = {}

---@alias NiriWorkspaces NiriWorkspace[]

---@class (exact) NiriWorkspace
---@field id number # Id of the workspace. Do not use this when trying to interract with niri
---@field unique_id number # The unique id of the workspace, used for interracting with niri
---@field active boolean # Whether the workspace is currently active. Every output can only have 1 active workspace
---@field focused boolean  # Whether the workspace is currently focused. Theres only 1 focused workspace across all outputs
---@field name? string # The name of the workspace, if it has one
---@field active_window_id? number # The id of the active window in this workspace.

---@alias NiriWindows NiriWindow[]

---@class (exact) NiriWindow
---@field id number # Unique id of the window, it remains constant while the window is opened.
---@field is_focused boolean # Whether this window is currently focused. There can only be 1 or 0 focused windows.
---@field title? string # Title of the window, if there is one
---@field app_id? string # Application ID, if there is one
---@field workspace_id? number # The id of the workspace this window is on, if any.

---Focuses on the specified workspace based on its unique id
---@param unique_workspace number # Unique id of the workspace to focus on
function services.niri:set_workspace(unique_workspace) end

---Gets the current active workspace
---@return integer
function services.niri:get_active_workspace() end

---Focuses on the workspace that is above the current one
function services.niri:focus_workspace_up() end

---Focuses on the workspace that is below the current one
function services.niri:focus_workspace_down() end

---Focuses on the specified window based on its unique id
---@param unique_window number # Unique id of the window to focus on
function services.niri:focus_window(unique_window) end

-- mpris service

---@class MprisService
services.mpris = {}

---@class (exact) MprisOutput
---@field player string # The players name

---@class (exact) MprisTrackChanged: MprisOutput
---@field track TrackMetadata | { empty: boolean }

---@class (exact) TrackMetadata
---@field title string # The title of the song playing
---@field album_name string # The album name of the song playing
---@field album_artists string[] # List of artists of the song
---@field length number # The length of the song in seconds

---@class (exact) MprisVolumeChanged: MprisOutput
---@field volume number

---@class (exact) MprisLoopingChanged: MprisOutput
---@field loop_status '"None"' | '"Track"' | '"Playlist"'

---@class (exact) MprisShuffleToggled: MprisOutput
---@field shuffle boolean

---@class MprisPlayer
local player = {}

---Sends a `play_pause` signal to the player
function player:play_pause() end

---Sends a `next` signal to the player
function player:next() end

---Sends a `previous` signal to the player
function player:previous() end

---Sets the volume of the player
---@param volume number
function player:set_volume(volume) end

---Sets the looping of the player
---@param status '"Playlist"' | '"Track"' | '"None"'
function player:set_loop(status) end

---Sets the shuffle of the player
---@param shuffle boolean
function player:set_shuffle(shuffle) end

---Gets the volume of the player
---@return number
function player:get_volume() end
---Gets a player by its name, allowing you to manage it
---@param player_name string
---@return MprisPlayer
function services.mpris:get_player(player_name) end

-- calls service

---@class CallsService
services.calls = {}

-- applications

services.applications = {}

---@class (exact) AppModel
---@field name string # The name of the app
---@field executable string # The apps executable binary path
---@field icon string # The icon name of the app
---@field description string # The description of the app, if it has one

---Gets all applications within .desktop
---@return AppModel[]
function services.applications:get_all_apps() end

---Launches an application, using the executable path
---@param executable_path string
function services.applications:launch_app(executable_path) end

---@class TimeService
services.time = {}

---Fires a signal after a set amount of delay
---@param duration number # The amount of seconds to wait
---@param signal CustomSignal | string # The signal name or custom signal to fire after the delay ends
function services.time:delay(duration, signal) end

---@class SystemTrayService
services.system_tray = {}

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

return services
