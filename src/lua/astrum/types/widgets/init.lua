---@meta

-- IMPORTANT: right now stuff like widgets dont have the best documentation
-- in the future document everything proprely so that everything is clear

---@class Widgets
local widgets = {}

-- most common parameters for an element

---@class Element
---@field width? Length
---@field height? Length
---@field horizontal_alignment? Horizontal
---@field vertical_alignment? Vertical

---@class CommonContainerElement
---@field width? Length
---@field height? Length

---@class (exact) CustomSignal
---@field signal_name string
---@field signal_data table # Data to be sent through the signal
-- window

---@class (exact) WindowModel
---@field name string,
---@field anchor Anchor[]

---comment
---@param model WindowModel
---@return Widgets
function widgets:window(model) end

-- text

---@class (exact) TextModel: Element # to be documented
---@field content string # text to be displayed

---comment
---@param model TextModel | string
---@return table
function widgets:text(model) end

-- button

---@class (exact) ButtonModel: CommonContainerElement
---@field child table # A widget that will be displayed within the button.
---@field padding? Padding
---@field on_press? CustomSignal | string # The name of the signal to be fired when a button is pressed
---@field on_scroll_up? CustomSignal | string # The name of the signal  to be fired when a button is scrolled up via the mouse wheel
---@field on_scroll_down? CustomSignal | string # The name of the signal  to be fired when a button is scrolled down via the mouse wheel

---comment
---@param model ButtonModel
---@return table
function widgets:button(model) end

-- row

---@class (exact) RowModel: CommonContainerElement
---@field children Element[] # List of elements to be rendered within the row
---@field spacing? number # The spacing of elements in pixels
---@field padding? Padding
---@field align_items? Alignment

--- to be doucmented
---@param model RowModel
---@return table
function widgets:row(model) end

-- container

---@class (exact) Container: CommonContainerElement
---@field child table # Element to be displayed within the container
---@field padding? Padding
---@field max_width? number # Maximum width in pixels
---@field max_height? number # Maximum height in pixels
---@field center_x? boolean # Should the contents of the container be centered horizontally?
---@field center_y? boolean # Should the contents of the container be centered vertically?
---@field horizontal_alignment? Horizontal
---@field vertical_alignment? Vertical

---to be documented
---@param model Container
---@return table
function widgets:container(model) end

return widgets
