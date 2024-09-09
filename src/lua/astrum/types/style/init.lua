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

---Returns a `RGBA8` value, used for defining colors, from a hex string
---@param hex string # The `hex` string that will be converted to `rgba`
---@return rgba
function style:hex_to_rgba(hex) end
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

---Adds a style that can be reused later
---@param class_name string # The name of the style
---@param style_settings table # The settings of the style, if there are multiple appearances in the style, only `default` is nesscessary
function style:add_style(class_name, style_settings) end

---Gets the style by its class name
---@param class_name string # The name of the style
---@return table
function style:get_style(class_name) end

---Sets the default font for text
---@param font_name string # The font's name
---@param font_size? number # The fonts size in pixels
---@param font_weight? FontWeight # The fonts weight
---@param font_style? FontStyle # The fonts style
function style:default_font(font_name, font_size, font_weight, font_style) end

---Gets a specific font
---@param font_name string # The font's name
---@param font_weight? FontWeight # The fonts weight
---@param font_style? FontStyle # The fonts style
---@return Font
function style:get_font(font_name, font_weight, font_style) end

-- TODO: ADD GRADIENT SUPPORT WITHIN BUTTON BACKGROUNDS

-- button

---@class (exact) ButtonAppearance
---@field text_color? rgba # The color of the text
---@field icon_color? rgba # The icon color of the button
---@field background? rgba # The background of the button
---@field overlay? rgba # The background overlay of the button
---@field shadow_offset? Vector # The amount of shadow offset to be applied on the button
---@field border_radius? number | [ number, number, number, number ] # Sets the border radius of the button
---@field border_width? number # Sets the width of the border within the button
---@field border_color? rgba # The color of the button border
---@field outline_width? number # An outline placed around the button
---@field outline_color? rgba # Sets the color of the outline

-- text

---@class (exact) TextAppearance
---@field text_color? rgba # The color of the text

-- container

---@class (exact) Border
---@field color rgba # The color of the border
---@field width number # The width of the border
---@field radius number | [ number, number, number, number ] # The radius of the border

---@class (exact) Shadow
---@field color rgba # The color of the shadow
---@field offset Vector # The offset of the shadow
---@field blur_radius number # The blur radius of the shadow

-- TODO: add gradients to backgrounds

---@class (exact) ContainerAppearance
---@field text_color? rgba # The color of the text
---@field icon_color? rgba # The color of icons within the container
---@field background? rgba # The background of the container
---@field border? Border # The border of the container
---@field shadow? Shadow

-- text inpuit

---@class (exact) TextInputAppearance
---@field background? rgba # The color of the background
---@field text_color? rgba # The color of the text within the text input
---@field placeholder_color? rgba # The color of the placeholder text within the text input
---@field selected_text_color? rgba # The color of selected text within the text input
---@field selected_fill? rgba # The color of the selected text background within the text input
---@field border_radius? number # The border radius
---@field border_width? number # The border width
---@field border_color? rgba # The color of the border

return style
