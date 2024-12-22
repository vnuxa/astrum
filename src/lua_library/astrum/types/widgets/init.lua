---@meta

---@class Widgets
local widgets = {}

---@class Widget # A generic widget
---@field widget_name string

---@module "widgets.misc"

---@class WidthHeightWidget # A widget that has width and height
---@field width? Length # Sets the width of the widget
---@field height? Length # Sets the height of the widget

---@class TextModel: WidthHeightWidget
---@field content? string # The text that should be displayed
---@field size? number # The font size of the text
---@field font? Font # The font of the text

-- TODO: Document the fact that there can be shorthands for widgets (ussualy ones that dont hold elements)
--

---A widget that holds text
---@param content_or_model? string | TextModel # The text to be displayed (shorthand) or the model for the text
---@param extra_model? TextModel # Extra params for the shorthand. You do not need this if you provide a model in the first argument
---@return Widget
function widgets:text(content_or_model, extra_model) end

---@class ButtonModel: WidthHeightWidget
---@field child? Widget # A widget that will be displayed within the button
---@field padding? Padding
---@field on_press? CustomSignal | string # Sends a signal whenever the button is pressed. If a string is provided, it will send no data

---A container that emits a signal when pressed on
---@param content_or_model? string | ButtonModel # The text to be displayed (shorthand) or the model for the button
---@param extra_model? ButtonModel # The params for the shorthand. You do not need this if you provide a model in the first argument
---@return Widget
function widgets:button(content_or_model, extra_model) end

---@class CustomSignal
---@field signal_name string
---@field signal_data table

---Creates a signal that can send in custom data
---@param signal_name string
---@param signal_data table # Data to be sent through the signal
---@return CustomSignal
function widgets:signal(signal_name, signal_data) end

---@class RowModel: WidthHeightWidget
---@field children? Widget[] # List of widgets to be rendered within the `row`
---@field spacing? number # The spacing between elements in pixels
---@field padding? Padding
---@field align_y? Vertical # Sets the vertical alignments of the contents of the `row`
---@field clip? boolean # Sets whether the contents of the `row` should be clipped on overflow

---A container that distributes content horizontally
---@param model? RowModel
function widgets:row(model)
	local row = {}
	row.class_name = "row"

	---Adds a widget to be displayed in the `row`
	---@param widget Widget
	function row:push(widget) end

	return row
end

---@class ColumnModel: WidthHeightWidget
---@field children? Widget[] # List of widgets to be rendered within the `column`
---@field spacing? number # The spacing between elements in pixels
---@field padding? Padding
---@field align_x? Horizontal # Sets the vertical alignments of the contents of the `column`
---@field clip? boolean # Sets whether the contents of the `column` should be clipped on overflow
---@field max_width? number # Maximum width of the `column` in pixels

---A container that distributes content horizontally
---@param model? ColumnModel
function widgets:column(model)
	local column = {}
	column.class_name = "column"

	---Adds a widget to be displayed in the `column`
	---@param widget Widget
	function column:push(widget) end

	return column
end

---@class CenterboxModel: WidthHeightWidget
---@field left_child? Widget # Element to be displayed on the left side of the centerbox
---@field middle_child? Widget # Element to be displayed in the middle of the centerbox
---@field right_child? Widget # Element to be displayed on the right side of the centerbox
---@field padding? Padding
---@field spacing? number # The spacing of elements in pixels
---@field align_items? Alignment

---A container that can distribute content on the left, middle and right sides
---@param model? CenterboxModel
---@return Widget
function widgets:centerbox(model) end

---@class ImageModel: WidthHeightWidget
---@field content? string # A path to an image, this field is required
---@field content_fit? ContentFit # Sets how the content should be fit. Defaults to `contain`
---@field filter_method? FilterMethod
---@field border_radius? number | [ number, number, number, number ] # The border radius of the image
---@field rotation? [ '"floating"', number ] | [ '"solid"', number ] # Sets the rotation of the image. `floating` - element will float while rotating, layout will be the same prior to rotation (default). `solid` - element will be solid while rotating, layout will be adjusted to fit rotated content
---@field opacity? number # Sets the opacity of an image. It should be in `0.0 - 1.0` range

--- A frame that displays an iamge while keeping a speciifed aspect ratio
---@param content_or_model? string | ImageModel # The text to be displayed (shorthand) or the model for the image
---@param extra_model? ImageModel # The params for the shorthand. You do not need this if you provide a model in the first argument
---@return Widget
function widgets:image(content_or_model, extra_model) end

---@class IconModel: WidthHeightWidget # Lazily generated SVG/PNG image
---@field icon_name? string # The name of the icon. `icon_name` or `icon_path` is required.
---@field icon_path? string # The path to the icon. `icon_name` or `icon_path` is required.
---@field size? integer # The size of the icon.
---@field content_fit? ContentFit # Sets how the content should be fit.

--- A lazily generated, generic icon
---@param icon_name_or_model? string | IconModel # The icon name (shorthand) or the model for the icon
---@param extra_model? IconModel # Extra params for the shorthand. You do not need this if you already provided a model in the first argument
---@return Widget
function widgets:icon(icon_name_or_model, extra_model) end

---@class ContainerModel: WidthHeightWidget
---@field center_x? Length # Sets the width of the `container` and centers the content horizontally
---@field center_y? Length # Sets the height of the `container` and centers the content vertically
---@field child? Widget # Element to be displayed within the `container`
---@field padding? Padding
---@field max_width? number # Maximum width of the `container` in pixels
---@field max_height? number # Maximum height of the `container` in pixels
---@field align_x? Horizontal # Sets the alignment of content on the horizontal axis
---@field align_y? Vertical # Sets the alignment of content on the vertical axis

--- A container that can be decorated or used for alignment
---@param model ContainerModel
---@return Widget
function widgets:container(model) end

return widgets
