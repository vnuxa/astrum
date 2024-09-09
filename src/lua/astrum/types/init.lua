---@meta

---@class Astrum
local astrum = {
	---@module "widgets"
	widgets = {},
	---@module "services"
	services = {},
	---@module "style"
	style = {},
	---@module "animations"
	animations = {},
	---@module "utils"
	utils = {},
}

---@module "misc"

---@class (exact) Signals
---@field hyprland? HyprlandSignal[]
---@field mpris? MprisSignal[]
---@field calls? CallsSignal[]
---@field time? number[] # The duration of time in seconds it should send a signal every time

---@alias HyprlandSignal
---| '"workspaces"' # Sends a signal whenever the workspaces get changed. Returns a `HyprlandWorkspaces` type

---@alias MprisSignal
---| '"playing"' # Sends a signal when the first player is playing. Returns a `MprisOutput` class
---| '"paused"' # Sends a signal when the first player is paused. Returns a `MprisOutput` class
---| '"stopped"' # Sends a signal when the first player has stopped. Returns a `MprisOutput` class
---| '"volume_changed"' # Sends a signal when the first player volume has changed. Returns a `MprisVolumeChanged` class
---| '"looping_changed"' # Sends a signal when the first player's loop settings have changed. Returns a `MprisLoopingChanged` class
---| '"shuffle_toggled"' # Sends a signal when the first player's shuffle settings have toggled. Returns a `MprisShuffleToggled` class
---| '"track_changed"' # Sends a signal when the first player's track metadata has changed. Returns a `MprisTrackChanged` class

---@alias CallsSignal string # Sends a signal when the call recieved matches the one specified

---@alias AstrumElement table # An element that is generated from `astrum.widgets`

---@alias SignalNames # List of pre-defined signal names
---| '"hyprland_workspaces"'
---| '"mpris_playing"'
---| '"mpris_paused"'
---| '"mpris_volume_changed"'
---| '"mpris_looping_changed"'
---| '"mpris_shuffle_toggled"'
---| '"mpris_track_changed"'
---| '"time_changed"'
---| string

---@alias ServiceNames # List of available service names
---| "hyprland"
---| "mpris"
---| "calls"
---| "time"

---@class (exact) ApplicationModel

---@class (exact) WindowModel
---@field view fun(): AstrumElement # Logic that dictates how the window should look.
---@field signals? { [SignalNames]: fun(signal_data: table) } # A dictionary of signal names and the logic that will be processed when the signal will be fired
---@field anchor? Anchor[]
---@field is_popup? boolean
---@field exclusive_zone? integer # How much space should the window reserve, set it to `-1` if you want to ignore other layers
---@field keymode? Keymode
---@field layer? Layer

--- ** TO BE DOCUMENTED**
function astrum:application()
	local app = {}

	--- Creates a toplevel widget that holds widgets and processes signals
	---@param window_name string # The unique name of the window
	---@param window_model WindowModel # The core of the window
	function app:window(window_name, window_model) end

	---Subscribes to a service, making it fire signals that windows will process
	---@param service ServiceNames # The requested service
	---@param model string[] | number[] # Which signals should it subscribe to
	function app:subscribe(service, model) end

	return app
end

-- INFO: dont know if i can make regular windows be invsiible
-- but if i should try, let me know

---Toggles the visibility of a popup window
---@param window_name string
function astrum:toggle_window(window_name) end

return astrum
