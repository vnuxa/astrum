---@meta

---@class Astrum
local astrum = {
	---@module "widgets"
	widgets = {},
	---@module "services"
	services = {},
	---@module "animations"
	animations = {},
	---@module "utils"
	utils = {},
	---@module "style"
	style = {},
}

---@module "subscriptions"

---@alias WindowSignal { [string]: fun(signal_data: table) }

---@class WindowModel
---@field view fun(): Widget # Logic that dictates what widgets for the window to render
---@field signals? WindowSignal | WindowSignal[] # A dictionary of signal names and their respective logic that will be processed when the signal is called on. If a table value is provided, it will unpack it. If there are multiple signals with the same name, it will get overriden
---@field subscriptions? Subscriptions # Connects to an external processes by sending signals. All of the subscriptions are to be provided in a table
---@field anchor? Anchor[]
---@field is_popup? boolean
---@field exclusive_zone? integer | "ignore" # How much space should the window reserve, set it to `"ignore"` if you want it to ignore other layers
---@field keymode? Keymode
---@field layer? Layer
---@field height? number # If provided, manually set the height of the window

---Makes an astrum application that is to be returned
function astrum:application()
	---@class AstrumApp
	local app = {}

	---Creates a toplevel surface, that holds and processes signals and widgets
	---@param window_name string # The unique name of the window
	---@param window_model WindowModel # The model of the window
	function app:window(window_name, window_model) end

	return app
end

---Toggles the visibility of a popup window
---@param window_name string
function astrum:toggle_window(window_name) end

---@alias Keymode
---| '"none"' # Keyboard input cannot be recieved
---| '"on_demand"' # Can recieve keyboard input if focused
---| '"exclusive"' # Steal keyboard input on top and overlay layers

---@alias Anchor # Specifies which edges and corners a layer should be placed at in the anchor rectangle
---| '"left"'
---| '"right"'
---| '"top"'
---| '"bottom"'

---@alias Layer
---| '"top"'
---| '"bottom"'
---| '"background"'

return astrum
