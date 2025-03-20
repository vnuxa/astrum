---@meta

-- colors

---@class (exact) rgb
---@field red number # Value ranging from 0-255
---@field green number # Value ranging from 0-255
---@field blue number # Value ranging from 0-255

---@class (exact) rgba
---@field red number # Value ranging from 0-255
---@field green number # Value ranging from 0-255
---@field blue number # Value ranging from 0-255
---@field alpha number # Value ranging from 0.0-1.0

-- for widgets

---@class (exact) Vector
---@field x number # The X component of the vector
---@field y number # the Y component of the vector

---@class (exact) Border
---@field color? rgba # The color of the border
---@field width? number # The width of the border
---@field radius? number | [ number, number, number, number ] # The radius of the border

---@class (exact) Shadow
---@field color? rgba # The color of the shadow
---@field offset? Vector # The offset of the shadow
---@field blur_radius? number # The blur radius of the shadow
