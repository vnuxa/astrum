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

--- A container that displays children on top of each other.
---@param model_or_children StackModel | Widgets[] # The model for the stack or a list of widgets. The first element of this list will determine the intrinsic size of the stack, every other element will be rendered on top; on its own layer
---@return Widget
function widgets:stack(model_or_children)
	local stack = {}
	stack.class_name = "stack"

	---Adds a widget to the `stack` widget
	---@param widget Widget
	function stack:push(widget) end

	return stack
end

---A menu that appears on an element uppon user interraction, such as a right-click mouse operation.
---@param model_or_child ContextMenuModel | Widget # The model for the context menu or the underlying element for it
---@param tree? TreeWidget[] # The menu tree that will be displayed when the right mouse button has been pressed on the underlying element
---@return Widget
function widgets:context_menu(model_or_child, tree) end

---A tree can either be an item for a menu or a menu that holds other items or menus, depending if the `tree` field is active. It itself is not a widget and should only be used within other widgets
---@param model_or_item TreeWidget | Widget # The model for the tree or the core item of the tree.
---@param tree? TreeWidget[] # Defines the `tree` field for the model.
function widgets:tree(model_or_item, tree) end

return widgets
