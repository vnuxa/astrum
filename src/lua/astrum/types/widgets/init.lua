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

---Creates the logic nesscessary to make a custom signal, it is only meant to be used inside of functions that need to have a `CustomSignal` return type
---@param model CustomSignal
---@return CustomSignal
function widgets:custom_signal(model) end

-- window
-- IMPORTANT: document everything

---@class (exact) WindowModel
---@field view fun(): AstrumElement # Logic that dictates how the window should look.
---@field anchor? Anchor[]
---@field is_popup? boolean
---@field exclusive_zone? integer # How much space should the window reserve, set it to `-1` if you want to ignore other layers
---@field keymode? Keymode

--- TO BE DOCUMENTED!!!
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

-- column

---@class (exact) ColumnModel: CommonContainerElement
---@field children? Element[] # List of elements to be rendered within the column
---@field spacing? number # The spacing of elements in pixels
---@field padding? Padding
---@field max_width? number # Maximum width in pixels
---@field max_height? number # Maximum height in pixels
---@field align_items? Alignment

--- to be documented
---@param model ColumnModel
---@return table
function widgets:column(model) end

-- container

---@class (exact) ContainerModel: CommonContainerElement
---@field child table # Element to be displayed within the container
---@field padding? Padding
---@field max_width? number # Maximum width in pixels
---@field max_height? number # Maximum height in pixels
---@field center_x? boolean # Should the contents of the container be centered horizontally?
---@field center_y? boolean # Should the contents of the container be centered vertically?
---@field horizontal_alignment? Horizontal
---@field vertical_alignment? Vertical

---to be documented
---@param model ContainerModel
---@return table
function widgets:container(model) end

-- centerbox

---@class (exact) CenterboxModel: CommonContainerElement
---@field left_child? table # Element to be displayed on the left side of the centerbox
---@field middle_child? table # Element to be displayed in the middle of the centerbox
---@field right_child? table # Element to be displayed on the right side of the centerbox
---@field padding? Padding
---@field spacing? number # The spacing of elements in pixels
---@field align_items? Alignment

--- to be documented
---@param model CenterboxModel
---@return table
function widgets:centerbox(model) end

-- scrollable

---@class (exact) ScrollableModel: CommonContainerElement
---@field child table # Infinite amount of content to be displayed within the scrollable
---@field direction? Direction # The direction where the content will be scrolled

--- to be documented
---@param model ScrollableModel
---@return table
function widgets:scrollable(model) end

-- text input

---@class (exact) TextInputModel # A field that can be filled with text
---@field value string # The value of the text input model. Needs an external variable in order to change
---@field placeholder string # Placeholder text for the text input
---@field always_active? boolean # If enabled, makes it behave as if it were always focused
---@field width? Length
---@field line_height? LineHeight # Sets the line height of the
---@field password? boolean # If the text input should be a secure password input
---@field on_input? fun(text: string): CustomSignal | string  # Runs a signal when some text is typed in the text input
---@field on_toggle_edit? fun(is_editing: boolean): CustomSignal | string
---@field on_submit? CustomSignal # Sends a custom signal when the text input is focused and the enter key is pressed
---@field size? number # Sets the text size of the text input

--- to be documented
---@param model TextInputModel
---@return table
function widgets:text_input(model) end

-- images, pointless except for border radius
-- TODO: see if implementing images is actually useful or not
-- for now implement icons

-- -@class (exact) ImageModel: CommonContainerElement # A frame that displays an image while keeping aspect ratio
-- -@field image_path string # Absolute path to the image
-- -@field border_radius? number | [ number, number, number, number ] # Sets the border radius of the image.

-- icons

---@class (exact) IconModel: CommonContainerElement # Lazily-generated SVG/PNG image
---@field icon string # Can be either a icon name or a path to the image
---@field size? integer
---@field content_fit? ContentFit

---to be documented
---@param model IconModel | string
---@return table
function widgets:icon(model) end

return widgets
