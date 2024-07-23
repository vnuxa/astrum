-- i dont think this is used for types
local utils = require("astrum_utils")
local table_to_string

table_to_string = function(data)
	local builder_string = "{ "
	local first_iter = true
	for key, value in pairs(data) do
		if first_iter then
			if type(value) == "table" then
				builder_string = builder_string .. "\n" .. tostring(key) .. " = " .. table_to_string(value)
			elseif type(value) == "string" then
				builder_string = builder_string .. "\n" .. tostring(key) .. ' = "' .. tostring(value) .. '"'
			else
				builder_string = builder_string .. "\n" .. tostring(key) .. " = " .. tostring(value)
			end
			first_iter = false
		else
			if type(value) == "table" then
				builder_string = builder_string .. ", \n" .. tostring(key) .. " = " .. table_to_string(value)
			elseif type(value) == "string" then
				builder_string = builder_string .. ", \n" .. tostring(key) .. ' = "' .. tostring(value) .. '"'
			else
				builder_string = builder_string .. ", \n" .. tostring(key) .. " = " .. tostring(value)
			end
		end
	end
	builder_string = builder_string .. "\n" .. "}"
	-- print("ok so we got the table to string and its this!")
	-- print(builder_string)

	return builder_string
end

local lib = {
	widget = function()
		local helper = {}

		---comment
		---@param model TextModel | string
		---@return table
		function helper:text(model)
			if type(model) == "string" then
				return {
					widget_name = "text",
					content = model,
				}
			elseif type(model) == "table" then
				local text = {
					widget_name = "text",
					content = model.content,
				}
				local function fill_optional(optional_name)
					if model[optional_name] then
						text[optional_name] = model[optional_name]
					end
				end
				-- optionals
				fill_optional("width")
				fill_optional("height")
				fill_optional("vertical_alignment")
				fill_optional("horizontal_alignment")

				return text
			end
			return {}
		end

		---comment
		---@param model ButtonModel
		---@return table
		function helper:button(model)
			local button = {
				widget_name = "button",
				child = model.child,
			}

			local function fill_optional(optional_name)
				if model[optional_name] then
					button[optional_name] = model[optional_name]
				end
			end
			local function optional_signal(signal_name)
				if model[signal_name] then
					if type(model[signal_name]) == "table" then
						button[signal_name] = {
							signal_name = model[signal_name].signal_name,
							signal_data = table_to_string(model[signal_name].signal_data),
						}
					else
						button[signal_name] = model[signal_name]
					end
				end
			end
			-- optionals
			fill_optional("width")
			fill_optional("height")

			fill_optional("padding")

			optional_signal("on_press")
			optional_signal("on_scroll_up")
			optional_signal("on_scroll_down")

			return button
		end

		function helper:row(model)
			local row = {
				widget_name = "row",
				children = model.children,
			}

			local function fill_optional(optional_name)
				if model[optional_name] then
					row[optional_name] = model[optional_name]
				end
			end
			-- optionals
			fill_optional("width")
			fill_optional("height")

			fill_optional("padding")
			fill_optional("spacing")
			fill_optional("align_items")

			return row
		end

		function helper:container(model)
			local container = {
				widget_name = "container",
				child = model.child,
			}
			local function fill_optional(optional_name)
				if model[optional_name] then
					container[optional_name] = model[optional_name]
				end
			end
			-- optionals
			fill_optional("width")
			fill_optional("height")

			fill_optional("padding")
			fill_optional("max_width")
			fill_optional("max_height")
			fill_optional("center_x")
			fill_optional("center_y")
			fill_optional("horizontal_alignment")
			fill_optional("vertical_alignment")

			return container
		end

		return helper
	end,
	service = function()
		local helper = {}

		helper.hyprland = {}

		function helper.hyprland:set_workspace(workspace)
			utils.hyprland_set_workspace(workspace)
		end

		helper.mpris = {}

		function helper.mpris:get_player(player_name)
			local player = utils.mpris_get_player(player_name)
			-- dont know if fucntion is required
			function player:play_pause()
				player.player_play_pause()
			end
			function player:next()
				player.player_next()
			end
			function player:previous()
				player.player_previous()
			end

			function player:set_volume(volume)
				player.player_volume(volume)
			end
			function player:set_loop(status)
				player.player_looping(status)
			end
			function player:set_shuffle(shuffle)
				player.player_shuffle(shuffle)
			end

			function player:get_volume()
				return player.player_get_volume()
			end

			return player
		end
		return helper
	end,
}

return lib
