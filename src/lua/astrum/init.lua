require("types")

local helper = require("helper")

---@class Astrum
local astrum = {
	widgets = helper.widget(),
	services = helper.service(),
}

function astrum:Application(model)
	local app = {
		update_logic = model.update_logic,
		view_logic = model.view_logic,
	}
	if model.requested_signals then
		app.requested_signals = model.requested_signals
	end

	return app
end
return astrum
