require("types")

local helper = require("helper")
local utils = require("astrum_utils")

---@class Astrum
local astrum = {
	widgets = helper.widget(),
	services = helper.service(),
	style = helper.style(),
	animations = helper.animation(),
	utils = require("helper.utils"),
}

local check_style
-- local style_differences = { off = {}, on = {} }

--- this function checks every value
check_style = function(animation, style_false, style_true)
	local new_data = {}
	for key, value in pairs(style_false) do
		if type(value) == "number" then
			if style_true[key] then
				new_data[key] = animation:animate_value(style_false[key], style_true[key])
			else
				new_data[key] = value
			end
		elseif type(value) == "table" then
			if style_true[key] then
				new_data[key] = check_style(animation, style_false[key], style_true[key])
			else
				new_data[key] = value
			end
		end
	end
	return new_data
end
--- i think i should document that both values must be present in each style in order to be animated
--- since its how it will know the from and to values required
function astrum.animations:animate_style(animation, style_false, style_true)
	local style_off = astrum.style:get_style(style_false)
	local style_on = astrum.style:get_style(style_true)

	return check_style(animation, style_off, style_on)
end
function astrum:application()
	local app = {
		windows = {},
		requested_signals = {},
	}
	app.update_logic = function(signal_name, signal_data)
		for _, window in pairs(app.windows) do
			if window.signals[signal_name] then window.signals[signal_name](signal_data) end
		end
	end

	function app:window(window_name, window_data) app.windows[window_name] = window_data end
	function app:subscribe(service, request) app.requested_signals[service] = request end

	-- if model.style then
	-- 	app.style = model.style
	-- end

	return app
end

function astrum:toggle_window(window_name) utils.toggle_window_call(window_name) end

return astrum
