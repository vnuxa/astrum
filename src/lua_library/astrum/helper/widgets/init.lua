local widgets = {}
local table_to_string

table_to_string = function(data)
	local builder_string = "{ "
	local first_iter = true
	for key, value in pairs(data) do
		if first_iter then
			if type(value) == "table" then
				builder_string = builder_string .. "\n" .. tostring(key) .. " = " .. table_to_string(value)
			elseif type(value) == "string" then
				builder_string = builder_string .. "\n" .. tostring(key) .. ' = "' .. tostring(value) .. '"'
			else
				builder_string = builder_string .. "\n" .. tostring(key) .. " = " .. tostring(value)
			end
			first_iter = false
		else
			if type(value) == "table" then
				builder_string = builder_string .. ", \n" .. tostring(key) .. " = " .. table_to_string(value)
			elseif type(value) == "string" then
				builder_string = builder_string .. ", \n" .. tostring(key) .. ' = "' .. tostring(value) .. '"'
			else
				builder_string = builder_string .. ", \n" .. tostring(key) .. " = " .. tostring(value)
			end
		end
	end
	builder_string = builder_string .. "\n" .. "}"
	-- print("ok so we got the table to string and its this!")
	-- print(builder_string)

	return builder_string
end

function widgets:signal(signal_name, signal_data)
	return {
		signal_name = signal_name,
		signal_data = table_to_string(signal_data),
	}
end

function widgets:text(content, model)
	local text
	if type(content) == "table" then
		text = content
	else
		text = model or {}
		text.content = content or ""
	end
	text.widget_name = "text"

	if default_font then
		if text.font == nil then text.font = default_font end
		if default_font.size and text.size == nil then text.size = default_font.size end
	end
	-- if default_font then text.font = default_font end
	return text
end

function widgets:button(content, model)
	local button
	if type(content) == "string" then
		button = model or {}
		button.child = widgets:text(content)
	else
		button = content or {}
	end
	button.widget_name = "button"

	return button
end

function widgets:centerbox(model)
	local center_box = model
	-- makes it so that if the left middle or right childs are mtpy it makes an empty child
	-- empty child being text that has nothing in it
	if model.left_child == nil then center_box.left_child = { widget_name = "text", content = "" } end
	if model.middle_child == nil then center_box.middle_child = { widget_name = "text", content = "" } end
	if model.right_child == nil then center_box.right_child = { widget_name = "text", content = "" } end

	model.widget_name = "centerbox"

	return center_box
end

function widgets:row(model)
	local row = model or {}

	if row.children == nil then row.children = {} end

	function row:push(child) table.insert(row.children, child) end

	row.widget_name = "row"

	return row
end

function widgets:column(model)
	local column = model or {}

	if column.children == nil then column.children = {} end

	function column:push(child) table.insert(column.children, child) end

	column.widget_name = "column"

	return column
end

function widgets:image(content_or_model, model)
	local image
	if type(content_or_model) == "string" then
		image = model or {}
		image.content = content_or_model
	else
		image = content_or_model or {}
	end
	image.widget_name = "image"

	return image
end

function widgets:icon(icon_name_or_model, model)
	local icon
	if type(icon_name_or_model) == "string" then
		icon = model or {}
		icon.icon_name = icon_name_or_model
	else
		icon = icon_name_or_model or {}
	end
	icon.widget_name = "icon"

	return icon
end

function widgets:text_input(content, placeholder, model)
	local text_input
	if type(content) == "table" then
		text_input = content
	else
		text_input = model or {}
		text_input.value = content or ""
		text_input.placeholder = placeholder
	end
	if text_input.placeholder == nil then text_input.placeholder = "" end
	text_input.widget_name = "text_input"

	if default_font then text_input.font = default_font end
	return text_input
end

function widgets:container(model)
	local container = model or {}
	if not container.child then container.child = widgets:text() end
	container.widget_name = "container"

	return container
end

function widgets:space(width, height)
	return {
		widget_name = "space",
		width = width,
		height = height,
	}
end

function widgets:scrollable(model)
	model.widget_name = "scrollable"

	return model
end

function widgets:mouse_area(model)
	model.widget_name = "mouse_area"

	return model
end

function widgets:stack(model_or_children)
	local stack = {}
	if model_or_children.children == nil then
		stack.children = model_or_children
	else
		stack = model_or_children
	end

	function stack:push(child) table.insert(stack.children, child) end

	stack.widget_name = "stack"
	return stack
end

function widgets:context_menu(model_or_child, tree)
	local context
	if type(model_or_child) == "table" and model_or_child.widget_name == nil then
		print("using the first optioni for context menu")
		context = model_or_child
	else
		print("using the second option for context menu")
		context = {
			child = model_or_child,
			tree = tree,
		}
	end
	context.widget_name = "context_menu"
	print("the item type is: ", context.child.widget_name)
	print("tree should have ", #tree, "amoint of items")

	return context
end

function widgets:tree(model_or_item, children)
	local tree
	if type(model_or_item) == "table" and model_or_item.widget_name == nil then
		print("i think model is provided")
		tree = model_or_item
	else
		print("using 2nd pair where itme is provided, model type: ", type(model_or_item))
		tree = {
			item = model_or_item,
			children = children,
		}
	end
	tree.widget_name = "tree"

	return tree
end

function widgets:menu(tree)
	return {
		widget_name = "menu",
		children = tree,
	}
end

return widgets
