---@meta
local services = {}

-- hyprland
services.hyprland = {}

---@class (exact) HyprlandWorkspace
---@field id number
---@field active boolean

---@alias HyprlandWorkspaces HyprlandWorkspace[]

---@class (exact) HyprlandClient
---@field workspace_id number # The id of the workspace that the client/window is on
---@field at { x: number, y: number } # The x and y position where the client is on
---@field size { x: number, y: number } # the x and y size of the client
---@field initial_title string # The `initial_title` of the client
---@field initial_class string # The `initial_title` of the client
---@field class string # The class name of the client
---@field title string # The title of the client
---@field process_id number # The process id of the client
---@field floating boolean # Is this window floating or not

---@alias HyprlandClients table<number, HyprlandClient> # The exact type you get from a client update signal within hyprland

---Sets the active workspace to the one specified within hyprland
---@param workspace number
function services.hyprland:set_workspace(workspace) end

---Gets the current active workspace
---@return integer
function services.hyprland:get_active_workspace() end

---Gets the current active client
---@return HyprlandClient
function services.hyprland:get_active_client() end
-- mpris

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

---@class ApplicationsService
services.applications = {}

---@class (exact) AppModel
---@field name string # The name of the app
---@field executable string # The apps executable binary path
---@field icon string # The icon name of the app
---@field id string # The id of the app. The id is oobtained by the desktop file name
---@field desktop string # The .desktop file path
---@field description string # The description of the app, if it has one

---Gets all applications within .desktop
---@return AppModel[]
function services.applications:get_all_apps() end

---Launches an application, using the executable path
---@param executable_path string
function services.applications:launch_app(executable_path) end

---@class Greetd
services.greetd = {}

---Attempts to login via greetd
---@param username string # The username to log in with
---@param attempt string # The password you need to loog in with
---@param command string # The command to run if login was successfull
---@return "login_success" | "login_failure"
function services.greetd:login(username, attempt, command) end

return services
