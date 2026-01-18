---@class Widget # A generic widget
---@field widget_name string

---@class WidthHeightWidget # A widget that has width and height
---@field width? Length # Sets the width of the widget
---@field height? Length # Sets the height of the widget

---@class TextModel: WidthHeightWidget
---@field content? string # The text that should be displayed
---@field align_x? Horizontal # Sets the horizontal alignment of the text
---@field align_y? Vertical # Sets the vertical alignment of the text
---@field height? Length # Sets the height of the text
---@field line_height? number # Sets the line height in pixels
---@field size? number # The font size of the text
---@field font? Font # The font of the text
---@field style? TextAppearance # Sets the appearance of the text

---@class ButtonModel: WidthHeightWidget
---@field child? Widget # A widget that will be displayed within the button
---@field padding? Padding
---@field on_press? CustomSignal | string # Sends a signal whenever the button is pressed. If a string is provided, it will send no data
---@field style? ButtonAppearance # Sets the appearance of the button

---@class TextInputModel
---@field value? string # The text of the text input. Needs an external variable paired with `on_input` in order to change
---@field placeholder? string # Placeholder text for the text input
---@field always_active? boolean # If enabled, makes it behave as if it were always focused
---@field width? Length
---@field line_height? LineHeight # Sets the line height of the
---@field password? boolean # If the text input should be a secure password input
---@field on_input? string  # Runs a signal when some text is typed in the text input, sends `text` in the signal data which contains the new text. Cannot pass through custom signals
---@field on_submit? string | CustomSignal # Sends a custom signal when the text input is focused and the enter key is pressed
---@field size? number # Sets the text size of the text input
---@field style? TextInputAppearance # Sets the appearance of the text input

---@class CustomSignal
---@field signal_name string
---@field signal_data table

---@class RowModel: WidthHeightWidget
---@field children? Widget[] # List of widgets to be rendered within the `row`
---@field spacing? number # The spacing between elements in pixels
---@field padding? Padding
---@field align_y? Vertical # Sets the vertical alignments of the contents of the `row`
---@field clip? boolean # Sets whether the contents of the `row` should be clipped on overflow

---@class ColumnModel: WidthHeightWidget
---@field children? Widget[] # List of widgets to be rendered within the `column`
---@field spacing? number # The spacing between elements in pixels
---@field padding? Padding
---@field align_x? Horizontal # Sets the vertical alignments of the contents of the `column`
---@field clip? boolean # Sets whether the contents of the `column` should be clipped on overflow
---@field max_width? number # Maximum width of the `column` in pixels

---@class CenterboxModel: WidthHeightWidget
---@field left_child? Widget # Element to be displayed on the left side of the centerbox
---@field middle_child? Widget # Element to be displayed in the middle of the centerbox
---@field right_child? Widget # Element to be displayed on the right side of the centerbox
---@field padding? Padding
---@field spacing? number # The spacing of elements in pixels
---@field align_items? Alignment

---@class ImageModel: WidthHeightWidget
---@field content? string # A path to an image, this field is required
---@field content_fit? ContentFit # Sets how the content should be fit. Defaults to `contain`
---@field filter_method? FilterMethod
---@field border_radius? number | [ number, number, number, number ] # The border radius of the image
---@field rotation? [ '"floating"', number ] | [ '"solid"', number ] # Sets the rotation of the image. `floating` - element will float while rotating, layout will be the same prior to rotation (default). `solid` - element will be solid while rotating, layout will be adjusted to fit rotated content
---@field opacity? number # Sets the opacity of an image. It should be in `0.0 - 1.0` range

---@class IconModel: WidthHeightWidget # Lazily generated SVG/PNG image
---@field icon_name? string # The name of the icon. `icon_name` or `icon_path` is required.
---@field icon_path? string # The path to the icon. `icon_name` or `icon_path` is required.
---@field size? integer # The size of the icon.
---@field content_fit? ContentFit # Sets how the content should be fit.

---@class ContainerModel: WidthHeightWidget
---@field center_x? Length # Sets the width of the `container` and centers the content horizontally
---@field center_y? Length # Sets the height of the `container` and centers the content vertically
---@field child? Widget # Element to be displayed within the `container`
---@field padding? Padding
---@field max_width? number # Maximum width of the `container` in pixels
---@field max_height? number # Maximum height of the `container` in pixels
---@field align_x? Horizontal # Sets the alignment of content on the horizontal axis
---@field align_y? Vertical # Sets the alignment of content on the vertical axis
---@field style? ContainerAppearance # Sets the appearance of the container

---@class ScrollableModel: WidthHeightWidget
---@field child? Widget # Infinite amount of content to be displayed within the scrollable
---@field direction? Direction # The direction where the content will be scrolled

---@class MouseAreaModel
---@field child Widget # Element that determines the size of the mouse area
---@field on_press? CustomSignal # Sends a signal when the left mouse button has been pressed over a specified area.
---@field on_release? CustomSignal # Sends a signal when the left mouse button has been released over a specified area.
---@field on_drag? CustomSignal # Sends a signal when a drag has been initiated over a specified area.
---@field on_double_click? CustomSignal # Sends a signal when the left mouse button has been pressed twice over a specified area.
---@field on_enter? CustomSignal # Sends a signal when the mouse has entered a specified area
---@field on_exit? CustomSignal # Sends a signal when the mouse has left a specified area
---@field on_middle_press? CustomSignal # Sends a signal when the middle mouse button has been pressed over a specified area.
---@field on_scroll? string # Sends to a specified signal name, sends `direction` field in a table that can be either `up` or `down` (e.g. OnScrollSignal)

---@class OnScrollSignal
---@field direction "up" | "down"

---@class StackModel: WidthHeightWidget
---@field children Widget[] # The first element of this list will determine the intrinsic size of the stack, every other element will be rendered on top; on its own layer

---@class TreeWidget
---@field item Widget # The core item for the tree, primarily used when combined with other tree's
---@field tree? TreeWidget[] # If `tree` proprety does not exist, then this tree element will be treated as an item, if it does exist then it will be treated as a menu that holds other items or menus
---@field height? number # The height in pixels
---@field width ? number # The width in pixels

---@class ContextMenuModel
---@field child Widget # The underlying element
---@field tree TreeWidget[] # The menu tree that will appear when right mouse button has been clicked on the underlying element
