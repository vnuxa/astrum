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

--- Creates a custom signal that can send through data
---@param signal_name string
---@param signal_data? table # Data to be sent through the signal
---@return CustomSignal
function widgets:signal(signal_name, signal_data) end

-- window
-- IMPORTANT: document everything

-- text

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
---@field weight? FontWeight # The weight of the font
---@field style? FontStyle # The style of the font

---@class (exact) TextModel: Element # to be documented
---@field content? string # text to be displayed
---@field style? TextAppearance # The style of the text
---@field size? number # The font size of the text
---@field font? Font # The font of the text

---comment
---@param model TextModel | string
---@return TextModel
function widgets:text(model) end

-- button

---@class (exact) ButtonModel: CommonContainerElement
---@field child? table # A widget that will be displayed within the button.
---@field padding? Padding
---@field on_press? CustomSignal | string # The name of the signal to be fired when a button is pressed
---@field on_scroll_up? CustomSignal | string # The name of the signal  to be fired when a button is scrolled up via the mouse wheel
---@field on_scroll_down? CustomSignal | string # The name of the signal  to be fired when a button is scrolled down via the mouse wheel
---@field style? { active: ButtonAppearance, hovered: ButtonAppearance, pressed: ButtonAppearance, disabled: ButtonAppearance} # Declares the buttons appearance

---comment
---@param model ButtonModel
---@return ButtonModel
function widgets:button(model) end

-- row

---@class (exact) RowModel: CommonContainerElement
---@field children? Element[] # List of elements to be rendered within the row
---@field spacing? number # The spacing of elements in pixels
---@field padding? Padding
---@field align_items? Alignment
---@field push? fun(self, widget: table) # Adds a widget that will be displayed in the `row`

--- to be doucmented
---@param model RowModel
---@return RowModel
function widgets:row(model) end

-- column

---@class (exact) ColumnModel: CommonContainerElement
---@field children? AstrumElement[] # List of elements to be rendered within the column
---@field spacing? number # The spacing of elements in pixels
---@field padding? Padding
---@field max_width? number # Maximum width in pixels
---@field max_height? number # Maximum height in pixels
---@field align_items? Alignment
---@field push? fun(self, widget: table) # Adds a widget that will be displayed in the `column`

--- to be documented
---@param model ColumnModel
---@return ColumnModel
function widgets:column(model) end

-- container

---@class (exact) ContainerModel: CommonContainerElement
---@field child? table # Element to be displayed within the container
---@field padding? Padding
---@field max_width? number # Maximum width in pixels
---@field max_height? number # Maximum height in pixels
---@field center_x? boolean # Should the contents of the container be centered horizontally?
---@field center_y? boolean # Should the contents of the container be centered vertically?
---@field horizontal_alignment? Horizontal
---@field vertical_alignment? Vertical
---@field style? table # The appearance of the container

---to be documented
---@param model ContainerModel
---@return ContainerModel
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
---@return CenterboxModel
function widgets:centerbox(model) end

-- scrollable

---@class (exact) ScrollableModel: CommonContainerElement
---@field child? table # Infinite amount of content to be displayed within the scrollable
---@field direction? Direction # The direction where the content will be scrolled

--- to be documented
---@param model ScrollableModel
---@return ScrollableModel
function widgets:scrollable(model) end

-- text input

---@class (exact) TextInputModel # A field that can be filled with text
---@field value? string # The text of the text input. Needs an external variable paired with `on_input` in order to change
---@field placeholder? string # Placeholder text for the text input
---@field always_active? boolean # If enabled, makes it behave as if it were always focused
---@field width? Length
---@field line_height? LineHeight # Sets the line height of the
---@field password? boolean # If the text input should be a secure password input
---@field on_input? string  # Runs a signal when some text is typed in the text input, sends `text` in the signal data which contains the new text. Cannot pass through custom signals
---@field on_toggle_edit? string # Sends a signal when the input editing status is toggled, sends `is_editing` in the signal data, which is a `boolean`. Cannot pass through custom signals
---@field on_submit? CustomSignal # Sends a custom signal when the text input is focused and the enter key is pressed
---@field size? number # Sets the text size of the text input
---@field style? table # The appearance of the text input model. Only the default field is nesscessary as any field that is not declared will inherit the default

--- to be documented
---@param model TextInputModel
---@return TextInputModel
function widgets:text_input(model) end

-- images, pointless except for border radius
-- TODO: see if implementing images is actually useful or not
-- for now implement icons

-- -@class (exact) ImageModel: CommonContainerElement # A frame that displays an image while keeping aspect ratio
-- -@field image_path string # Absolute path to the image
-- -@field border_radius? number | [ number, number, number, number ] # Sets the border radius of the image.

-- icons

---@class (exact) IconModel: CommonContainerElement # Lazily-generated SVG/PNG image
---@field icon? string # Can be either a icon name or a path to the image
---@field size? integer # The size of the icons that will be looked up, if icon field is a icon name
---@field content_fit? ContentFit
---@field change_icon? fun(self, new_icon: string) # Changes the icon to the new icon. Can be either a icon name or path to the image

---to be documented
---@param model IconModel | string
---@return IconModel
function widgets:icon(model) end

---@class (exact) ImageModel: CommonContainerElement # A frame that displays an image
---@field path? string # Path to the image
---@field content_fit? ContentFit # Sets the content fit of the image, defaults to `"contain"`
---@field filter_method? FilterMethod
---@field border_radius? number | [ number, number, number, number ] # The border radius of the image

---to be documented
---@param model ImageModel | string
---@return ImageModel
function widgets:image(model) end

---@class (exact) SpaceModel: CommonContainerElement # An empty amount of space

--- to be documented
---@param width Length | nil  # The width of the empty amount of space. Defaults to `"shrink"`
---@param height Length | nil # The height of the empty emount of space. Defaults to `"shrink"`
function widgets:space(width, height) end

return widgets
