---@meta

---@class Widgets
local widgets = {}

---@module "widgets.misc"
---@module "widgets.models"

-- TODO: Document the fact that there can be shorthands for widgets (ussualy ones that dont hold elements)
--

---A widget that holds text
---@param content_or_model? string | TextModel # The text to be displayed (shorthand) or the model for the text
---@param extra_model? TextModel # Extra params for the shorthand. You do not need this if you provide a model in the first argument
---@return Widget
function widgets:text(content_or_model, extra_model) end

---A container that emits a signal when pressed on
---@param content_or_model? string | ButtonModel # The text to be displayed (shorthand) or the model for the button
---@param extra_model? ButtonModel # The params for the shorthand. You do not need this if you provide a model in the first argument
---@return Widget
function widgets:button(content_or_model, extra_model) end

--- A field which can be filled with text.
---@param content_or_model TextInputModel | string
---@param placeholder? string
---@param model? TextInputModel
---@return Widget
function widgets:text_input(content_or_model, placeholder, model) end

---Creates a signal that can send in custom data
---@param signal_name string
---@param signal_data table # Data to be sent through the signal
---@return CustomSignal
function widgets:signal(signal_name, signal_data) end

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

---A container that can distribute content on the left, middle and right sides
---@param model? CenterboxModel
---@return Widget
function widgets:centerbox(model) end

--- A frame that displays an iamge while keeping a speciifed aspect ratio
---@param content_or_model? string | ImageModel # The text to be displayed (shorthand) or the model for the image
---@param extra_model? ImageModel # The params for the shorthand. You do not need this if you provide a model in the first argument
---@return Widget
function widgets:image(content_or_model, extra_model) end

--- A lazily generated, generic icon
---@param icon_name_or_model? string | IconModel # The icon name (shorthand) or the model for the icon
---@param extra_model? IconModel # Extra params for the shorthand. You do not need this if you already provided a model in the first argument
---@return Widget
function widgets:icon(icon_name_or_model, extra_model) end

--- A container that can be decorated or used for alignment
---@param model ContainerModel
---@return Widget
function widgets:container(model) end

--- An empty amount of space, useful for when you want to fill an area with nothing
---@param width? Length  # The width of the empty amount of space. Defaults to `"shrink"`
---@param height? Length # The height of the empty emount of space. Defaults to `"shrink"`
---@return Widget
function widgets:space(width, height) end

--- A container that can vertically display an infinite amount of content with a scrollbar
---@param model ScrollableModel
---@return Widget
function widgets:scrollable(model) end

---Emits signals on mouse events over a specified area
---@param model MouseAreaModel
---@return Widget
function widgets:mouse_area(model) end

return widgets
