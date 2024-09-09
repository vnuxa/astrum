local utils = require("astrum_utils")

return {
	require("helper.utils.json"),
	ternary = function(cond, T, F)
		if cond then
			return T
		else
			return F
		end
	end,
	ternary_eval = function(cond, T, F, ...)
		if cond then
			return T(...)
		else
			return F(...)
		end
	end,

	file_to_string = function(file_path)
		local f = assert(io.open(file_path, "rb"))
		local content = f:read("*all")
		f:close()

		return content
	end,

	run_command = function(...) return utils.execute_command(...) end,
}
