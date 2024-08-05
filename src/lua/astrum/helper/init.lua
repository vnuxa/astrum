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

		function helper:custom_signal(model)
			if type(model) == "table" then
				return {
					signal_name = model.signal_name,
					signal_data = table_to_string(model.signal_data),
				}
			else
				return model
			end
		end

		---comment
		---@param model WindowModel
		---@return table
		function helper:window(model)
			local window = {}
			window.view = model.view

			local function fill_optional(optional_name)
				if model[optional_name] then
					window[optional_name] = model[optional_name]
				end
			end

			fill_optional("is_popup")
			fill_optional("anchor")
			fill_optional("keymode")
			fill_optional("exclusive_zone")

			return window
		end

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

		function helper:centerbox(model)
			local centerbox = {
				widget_name = "centerbox",
			}

			local function fill_optional(optional_name)
				if model[optional_name] then
					centerbox[optional_name] = model[optional_name]
				end
			end

			-- workaround to having invisible centerbox widgets
			local function fill_child_optional(optional_name)
				if model[optional_name] then
					centerbox[optional_name] = model[optional_name]
				else
					centerbox[optional_name] = {
						widget_name = "text",
						content = "",
					}
				end
			end

			-- optionals

			fill_child_optional("left_child")
			fill_child_optional("middle_child")
			fill_child_optional("right_child")

			fill_optional("width")
			fill_optional("height")

			fill_optional("padding")
			fill_optional("spacing")
			fill_optional("align_items")

			return centerbox
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

		function helper:column(model)
			local column = {
				widget_name = "column",
				children = model.children,
			}

			local function fill_optional(optional_name)
				if model[optional_name] then
					column[optional_name] = model[optional_name]
				end
			end
			-- optionals
			fill_optional("width")
			fill_optional("height")

			fill_optional("max_width")
			fill_optional("max_height")

			fill_optional("padding")
			fill_optional("spacing")
			fill_optional("align_items")

			return column
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

		function helper:scrollable(model)
			local scrollable = {
				widget_name = "scrollable",
				child = model.child,
			}

			local function fill_optional(optional_name)
				if model[optional_name] then
					scrollable[optional_name] = model[optional_name]
				end
			end
			-- optionals
			fill_optional("width")
			fill_optional("height")

			fill_optional("direction")

			return scrollable
		end

		function helper:text_input(model)
			local text_input = {
				widget_name = "text_input",
				placeholder = model.placeholder,
				value = model.value,
			}

			local function fill_optional(optional_name)
				if model[optional_name] then
					text_input[optional_name] = model[optional_name]
				end
			end

			local function optional_signal(signal_name)
				if model[signal_name] then
					if type(model[signal_name]) == "table" then
						text_input[signal_name] = {
							signal_name = model[signal_name].signal_name,
							signal_data = table_to_string(model[signal_name].signal_data),
						}
					else
						text_input[signal_name] = model[signal_name]
					end
				end
			end
			-- optionals
			fill_optional("width")
			fill_optional("line_height")

			fill_optional("always_active")
			fill_optional("password")

			optional_signal("on_input")
			optional_signal("on_toggle_edit")
			optional_signal("on_submit")

			fill_optional("size")

			return text_input
		end

		function helper:icon(model)
			local function is_path(text)
				if string.find(string.sub(text, 1, 2), "/") then
					return true
				else
					return false
				end
			end
			if type(model) == "string" then
				local text = {
					widget_name = "icon",
				}
				if is_path(model) then
					text.icon_path = model
				else
					text.icon_name = model
				end

				return text
			elseif type(model) == "table" then
				local text = {
					widget_name = "icon",
				}

				if is_path(model.icon) then
					text.icon_path = model.icon
				else
					text.icon_name = model.icon
				end

				local function fill_optional(optional_name)
					if model[optional_name] then
						text[optional_name] = model[optional_name]
					end
				end
				-- optionals
				fill_optional("width")
				fill_optional("height")
				fill_optional("content_fit")

				return text
			end
			return {}
		end

		return helper
	end,

	service = function()
		local helper = {}

		helper.hyprland = {}

		function helper.hyprland:set_workspace(workspace)
			utils.hyprland_set_workspace(workspace)
		end
		function helper.hyprland:get_active_workspace()
			return utils.hyprland_get_active()
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

		helper.applications = {}

		function helper.applications:get_all_apps()
			local all_apps = utils.get_all_applications()

			return all_apps
		end
		function helper.applications:launch_app(executable_path)
			utils.launch_application(executable_path)
		end

		return helper
	end,

	-- defines the style of everything
	--
	style = function()
		local helper = {}

		function helper:rgba(red, green, blue, alpha)
			local color = {
				red = red,
				green = green,
				blue = blue,
				alpha = alpha,
			}

			return color
		end

		return helper
	end,
}

return lib
