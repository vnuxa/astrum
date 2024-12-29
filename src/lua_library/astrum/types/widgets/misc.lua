---@meta

-- font stuff

---@alias FontWeight # The weight of some text
---| '"thin"'
---| '"extralight"'
---| '"light"'
---| '"normal"'
---| '"medium"'
---| '"semibold"'
---| '"bold"'
---| '"extrabold"'
---| '"black"'

---@alias FontStyle # The style of the font
---| '"normal"'
---| '"italic"'
---| '"oblique"'

---@class Font
---@field name string # The font's name
---@field weight? FontWeight # The font's weight
---@field style? FontStyle # The style of the font

-- padding

---@alias Padding number | [number, number] | [number, number, number, number]

---@alias Length # The strategy used to fill space in a specific dimension
---| '"fill"' # Fill all the remaining space
---| '"shrink"' # Fill the least amount of space
---| ['"fill_portion"', number] # Fill a portion of the remaining space relative to other elements.
--- For example, we have two elements: one with `{ "fill_portion", 2 }` and the other with `{ "fill_portion", 3 }
--- The first will get 2 portions of the avialable space, while the second one would only get 3
---| ['"fixed"', number] # Fill a fixed amount of space

-- alignment

--- @alias Horizontal # The horizontal alignment of some element
---| '"left"'
---| '"center"'
---| '"right"'

---@alias Vertical # The vertical alignment of some element
---| '"top"'
---| '"center"'
---| '"bottom"'

---@alias Alignment
---| '"start"' # Alignment at the start of the axis
---| '"center"' # Alignment at the center of the axis
---| '"end"' # Alignment at the end of the axis

-- content fit

---@alias ContentFit # The strategy used to fit the contents of a widget to its bounding box
---| '"contain"' # Scale as big as it can be without needing to crop or hide parts. Doesnt distort the image, but there may be whitespaces
---| '"cover"' # Scale the image to cover all of the bounding box, cropping off if needed. Doesnt distort the image, but it might crop off certain edges
---| '"fill"' # Distort the image so that the widget is 100% covered without cropping
---| '"none"' # Dont resize or scale the image at all
---| '"scale_down"' # Scale the image down, if its too big for the space, but never scale it up

---@alias FilterMethod # Image filtering strategy
---| '"linear"' # Bilinear interpolation
---| '"nearest"' # Nearest neighbor

-- line height

---@alias LineHeight
---| [ "relative", number ] # A factor of size of the text
---| [ "absolute", number ] # An absolute height in pixels
