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

local default_font = nil
local astrum_helper_init = {
	widget = function()
		local helper = {}

		function helper:signal(signal_name, signal_data)
			if signal_data then
				return {
					signal_name = signal_name,
					signal_data = table_to_string(signal_data),
				}
			end
			return signal_name
		end

		---comment
		---@param model WindowModel
		---@return table
		function helper:window(model)
			local window = {}
			window.view = model.view

			local function fill_optional(optional_name)
				if model[optional_name] then window[optional_name] = model[optional_name] end
			end

			fill_optional("is_popup")
			fill_optional("anchor")
			fill_optional("keymode")
			fill_optional("exclusive_zone")
			fill_optional("layer")

			return window
		end

		---comment
		---@param model TextModel | string
		---@return table
		function helper:text(model)
			if type(model) == "string" then
				local text = {
					widget_name = "text",
					content = model or "",
				}
				if default_font then
					local font = { name = default_font.name }
					if default_font.weight then font.weight = default_font.weight end
					if default_font.style then font.style = default_font.style end

					text.font = font

					if default_font.size then text.size = default_font.size end
				end

				return text
			elseif type(model) == "table" then
				local text = {
					widget_name = "text",
					content = model.content or "",
				}

				if default_font then
					local font = { name = default_font.name }
					if default_font.weight then font.weight = default_font.weight end
					if default_font.style then font.style = default_font.style end

					text.font = font

					if default_font.size then text.size = default_font.size end
				end

				local function fill_optional(optional_name)
					if model[optional_name] then text[optional_name] = model[optional_name] end
				end
				-- optionals
				fill_optional("width")
				fill_optional("height")
				fill_optional("vertical_alignment")
				fill_optional("horizontal_alignment")

				fill_optional("style")
				-- IMPORTANT: add these
				fill_optional("font")
				fill_optional("size")

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
				child = model.child or { widget_name = "text", content = "" },
			}

			local function fill_optional(optional_name)
				if model[optional_name] then button[optional_name] = model[optional_name] end
			end

			-- optionals
			fill_optional("width")
			fill_optional("height")

			fill_optional("padding")
			fill_optional("style")

			fill_optional("on_press")
			fill_optional("on_scroll_up")
			fill_optional("on_scroll_down")

			return button
		end

		function helper:centerbox(model)
			local centerbox = {
				widget_name = "centerbox",
			}

			local function fill_optional(optional_name)
				if model[optional_name] then centerbox[optional_name] = model[optional_name] end
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
				children = model.children or {},
			}

			local function fill_optional(optional_name)
				if model[optional_name] then row[optional_name] = model[optional_name] end
			end

			function row:push(child) table.insert(row.children, child) end

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
				children = model.children or {},
			}

			local function fill_optional(optional_name)
				if model[optional_name] then column[optional_name] = model[optional_name] end
			end

			function column:push(child, test) table.insert(column.children, child) end
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
				child = model.child or {},
			}
			local function fill_optional(optional_name)
				if model[optional_name] then container[optional_name] = model[optional_name] end
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

			fill_optional("style")

			return container
		end

		function helper:scrollable(model)
			local scrollable = {
				widget_name = "scrollable",
				child = model.child or {},
			}

			local function fill_optional(optional_name)
				if model[optional_name] then scrollable[optional_name] = model[optional_name] end
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
				placeholder = model.placeholder or "",
				value = model.value or "",
			}

			if default_font then
				local font = { name = default_font.name }
				if default_font.weight then font.weight = default_font.weight end
				if default_font.style then font.style = default_font.style end

				text_input.font = font

				if default_font.size then text_input.size = default_font.size end
			end

			local function fill_optional(optional_name)
				if model[optional_name] then text_input[optional_name] = model[optional_name] end
			end

			-- optionals
			fill_optional("width")
			fill_optional("line_height")

			fill_optional("always_active")
			fill_optional("password")

			fill_optional("on_input")
			fill_optional("on_toggle_edit")
			fill_optional("on_submit")

			fill_optional("size")
			fill_optional("font")

			fill_optional("style")

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
				function text:change_icon(data)
					if is_path(data) then
						text.icon_path = data
					else
						text.icon_name = data
					end
				end

				text:change_icon(model)

				return text
			elseif type(model) == "table" then
				local text = {
					widget_name = "icon",
				}

				function text:change_icon(data)
					if is_path(data) then
						text.icon_path = data
					else
						text.icon_name = data
					end
				end

				text:change_icon(model.icon)

				local function fill_optional(optional_name)
					if model[optional_name] then text[optional_name] = model[optional_name] end
				end
				-- optionals
				fill_optional("width")
				fill_optional("height")
				fill_optional("content_fit")
				fill_optional("size")

				return text
			end
			return {}
		end
		function helper:image(model)
			if type(model) == "string" then
				return {
					widget_name = "image",
					path = model,
				}
			elseif type(model) == "table" then
				local image = {
					widget_name = "image",
					path = model.path,
				}

				local function fill_optional(optional_name)
					if model[optional_name] then image[optional_name] = model[optional_name] end
				end
				-- optionals
				fill_optional("width")
				fill_optional("height")
				fill_optional("content_fit")
				fill_optional("filter_method")
				fill_optional("border_radius")

				return image
			end

			return {}
		end

		function helper:space(width, height)
			local space = {
				widget_name = "space",
			}
			if width then space.width = width end
			if height then space.height = height end

			return space
		end

		return helper
	end,

	service = function()
		local helper = {}

		helper.hyprland = {}

		function helper.hyprland:set_workspace(workspace) utils.hyprland_set_workspace(workspace) end
		function helper.hyprland:get_active_workspace() return utils.hyprland_get_active() end

		helper.mpris = {}

		function helper.mpris:get_player(player_name)
			local player = utils.mpris_get_player(player_name)
			-- dont know if fucntion is required
			function player:play_pause() player.player_play_pause() end
			function player:next() player.player_next() end
			function player:previous() player.player_previous() end

			function player:set_volume(volume) player.player_volume(volume) end
			function player:set_loop(status) player.player_looping(status) end
			function player:set_shuffle(shuffle) player.player_shuffle(shuffle) end

			function player:get_volume() return player.player_get_volume() end

			return player
		end

		helper.applications = {}

		function helper.applications:get_all_apps()
			local all_apps = utils.get_all_applications()

			return all_apps
		end
		function helper.applications:launch_app(executable_path) utils.launch_application(executable_path) end

		return helper
	end,

	-- defines the style of everything
	--
	style = function()
		local helper = {}
		helper.styles = {}

		function helper:rgba(red, green, blue, alpha)
			return {
				red = red,
				green = green,
				blue = blue,
				alpha = alpha,
			}
		end
		function helper:vector(x, y)
			return {
				x = x,
				y = y,
			}
		end

		function helper:hex_to_rgba(hex)
			hex = hex:gsub("#", "")
			return {
				red = tonumber("0x" .. hex:sub(1, 2)),
				green = tonumber("0x" .. hex:sub(3, 4)),
				blue = tonumber("0x" .. hex:sub(5, 6)),
				alpha = tonumber("0x" .. hex:sub(7, 8)) / 255,
			}
		end

		function helper:add_style(class_name, style_settings)
			if style_settings.default then
				local settings = {}

				for key, value in pairs(style_settings) do
					if key ~= "default" then
						-- replace the default nil values to the default values
						-- makes it easier to track what everything should look like
						settings[key] = value
						for k, v in pairs(style_settings.default) do
							if settings[key][k] == nil then settings[key][k] = v end
						end
					end
				end

				settings.default = style_settings.default
				helper.styles[class_name] = settings
			else
				helper.styles[class_name] = style_settings
			end
		end

		function helper:get_style(class_name)
			-- if style_name then
			-- 	return helper.styles[class_name][style_name]
			-- else
			return helper.styles[class_name]
			-- end
		end

		function helper:default_font(font_name, font_size, font_weight, font_style)
			local font = {
				name = font_name,
			}
			if font_size then font.size = font_size end
			if font_weight then font.weight = font_weight end
			if font_style then font.style = font_style end

			default_font = font
		end

		function helper:get_font(font_name, font_weight, font_style)
			local font = {}
			if font_name then
				font.name = font_name
			else
				if default_font then font.name = default_font.name end
			end

			if font_weight then font.weight = font_weight end

			if font_style then font.style = font_style end

			return font
		end

		return helper
	end,

	animation = function()
		local helper = {
			animation_id = 0,
		}

		function helper:new(starting_value, time, easing, repeat_amount, reverse, delay)
			local anim =
				helper:with_id(starting_value, time, easing, repeat_amount, reverse, delay, helper.animation_id)

			helper.animation_id = helper.animation_id + 1

			return anim
		end
		function helper:with_id(starting_value, time, easing, repeat_amount, reverse, delay, animation_id)
			local anim_info = {
				animation_id = animation_id,
				switch_states = false,
			}

			local function set_or(value_name, value, default)
				if value ~= nil then
					anim_info[value_name] = value
				else
					anim_info[value_name] = default
				end
			end

			set_or("value", starting_value, false)
			set_or("time", time, 1)
			set_or("easing", easing, "linear")
			set_or("repeat_count", repeat_amount, nil)
			set_or("reverse", reverse, false)
			set_or("delay", delay, nil)

			utils.make_animation(anim_info)
			local anim = {}

			-- from value is when tis false
			-- to value is when tis true
			function anim:animate_value(from_value, to_value)
				if anim_info.switch_states then
					return utils.animate_value({
						animation_id = anim_info.animation_id,
						from_value = to_value,
						to_value = from_value,
					})
				end
				return utils.animate_value({
					animation_id = anim_info.animation_id,
					from_value = from_value,
					to_value = to_value,
				})
			end
			local check_table
			check_table = function(from_table, to_table)
				local new_data = {}
				for key, value in pairs(from_table) do
					if type(value) == "number" then
						if to_table[key] then
							new_data[key] = anim:animate_value(from_table[key], to_table[key])
						else
							new_data[key] = value
						end
					elseif type(value) == "table" then
						if to_table[key] then
							new_data[key] = check_table(from_table[key], to_table[key])
						else
							new_data[key] = value
						end
					end
				end
				return new_data
			end

			function anim:animate_table(from_table, to_table) return check_table(from_table, to_table) end

			function anim:toggle(value)
				anim_info.value = utils.set_animation({ animation_id = anim_info.animation_id, value = value })
			end

			function anim:play()
				print("got play request...")
				-- anim_info.value = utils.set_animation({ animation_id = anim_info.animation_id, value = value })
				-- utils.change_anim_state({ animation_id = anim_info.animation_id, value = false })
				anim_info.switch_states = anim_info.value
				anim:toggle()
			end

			return anim
		end

		return helper
	end,
}

return astrum_helper_init
