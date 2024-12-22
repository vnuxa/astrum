local utils = require("astrum_core_utils")

return {
	json = require("astrum.helper.utils.json"),
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
	match = function(scrutinee)
		local lib = {}

		function lib.arm(condition, result)
			if lib.result then return lib end
			if scrutinee == condition then lib.result = result end
			return lib
		end

		function lib.arm_less_eq(condition, result)
			if lib.result then return lib end
			if scrutinee <= condition then lib.result = result end
			return lib
		end

		function lib.arm_more_eq(condition, result)
			if lib.result then return lib end
			if scrutinee >= condition then lib.result = result end
			return lib
		end

		function lib.arm_less_than(condition, result)
			if lib.result then return lib end
			if scrutinee < condition then lib.result = result end
			return lib
		end
		function lib.arm_more_than(condition, result)
			if lib.result then return lib end
			if scrutinee > condition then lib.result = result end
			return lib
		end

		function lib.default(result)
			lib.fallback = result
			return lib
		end

		function lib.collapse()
			if lib.result then
				return lib.result
			elseif lib.fallback then
				return lib.fallback
			end
		end

		return lib
	end,
	switch = function(scrutinee)
		local lib = {}

		function lib.case(condition, result)
			if lib.result then return lib end
			if scrutinee == condition then lib.result = result end
			return lib
		end

		function lib.case_less_eq(condition, result)
			if lib.result then return lib end
			if scrutinee <= condition then lib.result = result end
			return lib
		end

		function lib.case_more_eq(condition, result)
			if lib.result then return lib end
			if scrutinee >= condition then lib.result = result end
			return lib
		end

		function lib.case_less_than(condition, result)
			if lib.result then return lib end
			if scrutinee > condition then lib.result = result end
			return lib
		end
		function lib.case_more_than(condition, result)
			if lib.result then return lib end
			if scrutinee < condition then lib.result = result end
			return lib
		end

		function lib.default(result) lib.fallback = result end

		function lib.collapse()
			if lib.result then
				lib.result()
			elseif lib.fallback then
				lib.fallback()
			end
		end

		return lib
	end,
	file_to_string = function(file_path)
		local f = assert(io.open(file_path, "rb"))
		local content = f:read("*all")
		f:close()

		return content
	end,

	run_command = function(...) return utils.execute_command(...) end,
}
