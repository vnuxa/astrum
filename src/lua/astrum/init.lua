require("types")

local helper = require("helper")
local utils = require("astrum_utils")

---@class Astrum
local astrum = {
	widgets = helper.widget(),
	services = helper.service(),
	style = helper.style(),
}

function astrum:application(model)
	local app = {
		update_logic = model.update_logic,
		windows = model.windows,
	}
	if model.requested_signals then
		app.requested_signals = model.requested_signals
	end
	if model.style then
		app.style = model.style
	end

	return app
end

function astrum:toggle_window(window_name)
	utils.toggle_window_call(window_name)
end

return astrum
