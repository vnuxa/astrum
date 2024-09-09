---@meta
--- TODO: Documnt all of these

---@alias Keymode
---| '"none"' # Keyboard input cannot be recieved
---| '"on_demand"' # Can recieve keyboard input if focused
---| '"exclusive"' # Steal keyboard input on top and overlay layers

---@alias Anchor # Specifies which edges and corners a layer should be placed at in the anchor rectangle
---| '"left"'
---| '"right"'
---| '"top"'
---| '"bottom"'

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

-- padding

---@alias Padding number | [number, number] | [number, number, number, number]

-- alignment

---@alias Alignment
---| '"start"' # Alignment at the start of the axis
---| '"center"' # Alignment at the center of the axis
---| '"end"' # Alignment at the end of the axis

-- scrollable

---@alias ScrollableAlignment
---| '"start"' # Content is aligned to the start of the `Scrollable`
---| '"end"' # Content is aligned to the end of the `Scrollable`

---@class (exact) ScrollablePropreties # The scrollbars propreties within a `Scrollable`
---@field width? number # Sets the width of the scrollbar in pixels
---@field margin? number # Sets the scrollbar margin in pixels
---@field scrollers_width? number # Sets the width of the scroller in pixels
---@field alignment? ScrollableAlignment # Sets the alignment of the scrollable

---@alias Direction
---| [ '"vertical"', ScrollablePropreties ] # Vertical scrolling
---| [ '"horizontal"', ScrollablePropreties ] # Horizontal scrolling
---| [ '"both"', { vertical: ScrollablePropreties, horizontal: ScrollablePropreties }] # Both horizontal and vertical scrolling

-- line height

---@alias LineHeight
---| [ "relative", number ] # A factor of size of the text
---| [ "absolute", number ] # An absolute height in pixels

-- content fit

---@alias ContentFit # The strategy used to fit the contents of a widget to its bounding box
---| '"contain"' # Scale as big as it can be without needing to crop or hide parts. Doesnt distort the image, but there may be whitespaces
---| '"cover"' # Scale the image to cover all of the bounding box, cropping off if needed. Doesnt distort the image, but it might crop off certain edges
---| '"fill"' # Distort the image so that the widget is 100% covered without cropping
---| '"none"' # Dont resize or scale the image at all
---| '"scale_down"' # Scale the image down, if its too big for the space, but never scale it up

-- filter method

---@alias FilterMethod # Image filtering strategy
---| '"linear"' # Bilinear interpolation
---| '"nearest"' # Nearest neighbor

-- layer

---@alias Layer
---| '"top"'
---| '"bottom"'
---| '"background"'
