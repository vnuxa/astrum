local style = {
	styles = {},
}
default_font = nil

function style:rgba(red, green, blue, alpha)
	return {
		red = red,
		green = green,
		blue = blue,
		alpha = alpha,
	}
end

function style:vector(x, y)
	return {
		x = x,
		y = y,
	}
end

function style:hex_to_rgb(hex)
	hex = hex:gsub("#", "")
	return {
		red = tonumber("0x" .. hex:sub(1, 2)),
		green = tonumber("0x" .. hex:sub(3, 4)),
		blue = tonumber("0x" .. hex:sub(5, 6)),
	}
end

function style:hex_to_rgba(hex)
	hex = hex:gsub("#", "")
	if string.len(hex) == 6 then
		return {
			red = tonumber("0x" .. hex:sub(1, 2)),
			green = tonumber("0x" .. hex:sub(3, 4)),
			blue = tonumber("0x" .. hex:sub(5, 6)),
			alpha = 1,
		}
	else
		return {
			red = tonumber("0x" .. hex:sub(1, 2)),
			green = tonumber("0x" .. hex:sub(3, 4)),
			blue = tonumber("0x" .. hex:sub(5, 6)),
			alpha = tonumber("0x" .. hex:sub(7, 8)) / 255,
		}
	end
end

function style:add_style(class_name, style_settings)
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
		style.styles[class_name] = settings
	else
		style.styles[class_name] = style_settings
	end
end

style.add_button_style = style.add_style
style.add_text_style = style.add_style
style.add_container_style = style.add_style
style.add_text_input_style = style.add_style

function style:get_style(class_name) return style.styles[class_name] end

function style:default_font(font_name, font_size, font_weight, font_style)
	local font = {
		name = font_name,
	}
	if font_size then font.size = font_size end
	if font_weight then font.weight = font_weight end
	if font_style then font.style = font_style end

	default_font = font
end

function style:get_font(font_name, font_weight, font_style)
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

return style
