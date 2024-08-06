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

-- widgets

---@class (exact) Vector
---@field x number # The X component of the vector
---@field y number # the Y component of the vector

---Makes a `2D Vector`
---@param x number # The X value of the vector
---@param y number # The Y value of the vector
---@return Vector
function style:vector(x, y) end

-- TODO: ADD GRADIENT SUPPORT WITHIN BUTTON BACKGROUNDS

-- button

---@class (exact) ButtonAppearance
---@field text_color? rgba # The color of the text
---@field icon_color? rgba # The icon color of the button
---@field background? rgba # The background of the button
---@field overlay? rgba # The background overlay of the button
---@field shadow_offset Vector # The amount of shadow offset to be applied on the button
---@field border_radius number | [ number, number, number, number ] # Sets the border radius of the button
---@field border_width number # Sets the width of the border within the button
---@field border_color rgba # The color of the button border
---@field outline_width number # An outline placed around the button
---@field outline_color rgba # Sets the color of the outline

-- text

---@class (exact) TextAppearance
---@field text_color rgba # The color of the text

return style
