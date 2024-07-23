local astrum = require("lua.init")
local widgets = astrum.widgets

return {
	update_logic = function(signal_name, signal_data)
		print(signal_name)
		print("signal_data: ")
		for key, value in pairs(signal_data) do
			print("     ", key, value)
		end
	end,
	view_logic = function()
		widgets:text("hello")
		-- widgets:text("helloo")
		-- print("view logiccc wow")
	end,
	-- it should nto be a table of strings
	-- shoudl look like this
	-- requsted_services = {
	--      hyprland = {
	--          "workspaces"
	--      }
	--      calls = {
	--          "call_name"
	--      }
	-- }
	--
	--you get it
	--you can get the call name with this veryy yeasil!!

	requested_services = {
		"hyprland_workspaces",
	},
}
