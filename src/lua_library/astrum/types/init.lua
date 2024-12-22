---@meta

---@class Astrum
local astrum = {
	---@module "widgets"
	widgets = {},
	---@module "services"
	services = {},
	---@module "animations"
	animations = {},
}

---@module "subscriptions"

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

function astrum:application()
	local app = {}

	---Creates a toplevel surface, that holds and processes signals and widgets
	---@param window_name string # The unique name of the window
	---@param window_model WindowModel # The model of the window
	function app:window(window_name, window_model) end

	return app
end

return astrum
