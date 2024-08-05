---@meta

local style = {}

-- colors

---@class (exact) rgba
---@field red number # Value ranging from 0-255
---@field green number # Value ranging from 0-255
---@field blue number # Value ranging from 0-255
---@field alpha number # Value ranging from 0.0-1.0

---Returns a `RGBA8` value, used for defining colors
---@param red number # Any value between 0-255 that defines the color `red`
---@param green number # Any value between 0-255 that defines the color `green`
---@param blue number # Any value between 0-255 that defines the color `blue`
---@param alpha number # A floating point integer, ranging from 0.0-1.0 that defines the `transparency`
---@return rgba
function style:rgba(red, green, blue, alpha) end

-- INFO: maybe i should make a style:hex that transforms it into an rgba value?

-- application

---@class (exact) ApplicationAppearance
---@field background_color rgba # The background `color` of the application
---@field icon_color rgba # The default icon `color` of the application
---@field text_color rgba # The default text `color` of the application

return style
