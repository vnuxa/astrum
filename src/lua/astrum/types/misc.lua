---@meta
--- TODO: Documnt all of these

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
