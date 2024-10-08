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

return services
