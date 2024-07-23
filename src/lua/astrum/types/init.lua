---@meta

---@class Astrum
local astrum = {
	---@module "widgets"
	widgets = {},
	---@module "services"
	services = {},
}

---@module "misc"

-- INFO:
-- ok so services should be redone this way
-- it will be a table of functions
-- each function is needed because it will tell process it from a subscribtion thing to a signal
-- signals are essentialy:
--   signal_name: string
--   signal_data: table (contents depend on the subscription, but it allows it ot be dynamic)
-- sooo a function would be like
-- astrum.services({
--      workspaces_changed = "something_like_workspaces_changed" -- the user tells the signal name btw!!!
--      -- and eveyrthing that is not included will be ignored
-- })
-- everything that is included will only tell that it shouldnt ignore this event
-- and then it sends a signal, with the signal name that the user defined and the data that is obtained through thr eservice

---@class (exact) Signals
---@field hyprland? HyprlandSignal[]
---@field mpris? MprisSignal[]
---@field calls? CallsSignal[]

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

---@alias AstrumElement table

---@alias SignalNames # List of pre-defined signal names
---| string
---| '"hyprland_workspaces"'
---| '"mpris_playing"'
---| '"mpris_paused"'
---| '"mpris_volume_changed"'
---| '"mpris_looping_changed"'
---| '"mpris_shuffle_toggled"'
---| '"mpris_track_changed"'

---@class (exact) ApplicationModel
---@field update_logic fun(signal_name: SignalNames, signal_data: table) # Logic that is sent when a signal needs to be processed
---@field view_logic fun(): AstrumElement # Logic that returns how the application should be rendered. Function gets run after processing update_logic
---@field requested_signals? Signals

--- ** TO BE DOCUMENTED**
---@param model ApplicationModel
function astrum:Application(model) end

return astrum
