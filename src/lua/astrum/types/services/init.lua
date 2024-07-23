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

-- mpris service

---@class MprisService
services.mpris = {}

---@class (exact) MprisOutput
---@field player string # The players name

---@class (exact) MprisTrackChanged: MprisOutput
---@field track TrackMetadata

---@class (exact) TrackMetadata
---@field album_name string
---@field album_artists string[] list of artists
---@field length number the length of the song in seconds

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

return services
