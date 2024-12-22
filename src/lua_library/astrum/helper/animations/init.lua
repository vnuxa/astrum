local utils = require("astrum_core_utils")

local animations = {
	animation_id = 0,
}

function animations:new(starting_value, time, easing, repeat_amount, reverse, delay)
	local anim_info = {
		animation_id = animations.animation_id,
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

	-- if the state is already on a true,
	-- animate it back to the false value
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

	function anim:get_state() return anim_info.value end

	function anim:play()
		-- anim_info.switch_states = anim_info.value
		anim:toggle()
	end

	animations.animation_id = animations.animation_id + 1

	return anim
end

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

---Transitions all the number values from one style to the other one based on the animation state
---@param animation Animation
---@param style_false table
---@param style_true table
---@return table
function animations:animate_style(animation, style_false, style_true)
	local style_off = astrum.style:get_style(style_false)
	local style_on = astrum.style:get_style(style_true)

	return check_style(animation, style_off, style_on)
end

return animations
