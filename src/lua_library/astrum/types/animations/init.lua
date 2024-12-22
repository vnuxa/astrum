---@meta

local animations = {}

---@alias EasingStyle
---| '"linear"'
---| '"ease_in"'
---| '"ease_out"'
---| '"ease_in_out"'
---| '"ease_in_quad"'
---| '"ease_out_quad"'
---| '"ease_in_out_quad"'
---| '"ease_in_cubic"'
---| '"ease_out_cubic"'
---| '"ease_in_out_cubic"'
---| '"ease_in_quart"'
---| '"ease_out_quart"'
---| '"ease_in_out_quart"'
---| '"ease_in_quint"'
---| '"ease_out_quint"'
---| '"ease_in_out_quint"'
---| '"ease_in_expo"'
---| '"ease_out_expo"'
---| '"ease_in_out_expo"'
---| '"ease_in_circ"'
---| '"ease_out_circ"'
---| '"ease_in_out_circ"'
---| '"ease_in_back"'
---| '"ease_out_back"'
---| '"ease_in_out_back"'
---| '"ease_in_elastic"'
---| '"ease_out_elastic"'
---| '"ease_in_out_elastic"'
---| '"ease_in_bounce"'
---| '"ease_out_bounce"'
---| '"ease_in_out_bounce"'

---Makes a new animation based on the provided settings
---@param starting_value boolean | nil # The state of the animation, default is `false`
---@param time number | nil # The amount of seconds it takes for the animation to complete. Default is `1`
---@param easing EasingStyle | nil # The easing style of the animation, default is `"linear"`
---@param repeat_amount number | nil # The amount of times the animation will repeat itself, default is `nil`
---@param reverse boolean | nil # Will the animation reverse itself, default is `false`
---@param delay number | nil # The delay until the animation will start, in seconds. Default is `0`
function animations:new(starting_value, time, easing, repeat_amount, reverse, delay)
	---@class Animation
	---@field animation_id number # The animation's ID, used for interracting with lilt animations
	local animation = {}

	---Animates any number value, based on the animation state
	---@param false_value number # The value to transition to when the `animation state` is `false`
	---@param true_value number # The value to transition to when the `animation state` is `true`
	function animation:animate_value(false_value, true_value) end

	---Animates all of the tables number values, based on the animation's state
	---@param false_table table # The table to go to when the `animation state` is `false`
	---@param true_table table # The table to go to when the `animation state` is `true`
	function animation:animate_table(false_table, true_table) end

	---Changes the animation state without playing an animation
	---@param state? boolean # If provided, change to a specific state
	function animation:toggle(state) end

	---Plays an animation  and toggles the state
	function animation:play() end

	---Gets the current animation state
	---@return boolean
	function animation:get_state() end

	return animation
end

---Animates a widget's style, based on the animation's state. Both provided styles must have a number value in order to animate it
---@param animation Animation
---@param false_style string # The classname of the requested style to go to when the `animation` is `false`.
---@param true_style string # The classname of the requested style to go to when the `animation` is `true`.
function animations:animate_style(animation, false_style, true_style) end

return animations
