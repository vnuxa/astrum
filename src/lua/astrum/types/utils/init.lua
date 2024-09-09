---@meta

local lib = {}

local json = {}
---Converts a lua table into a json string
---@param value table
---@return string
function json:encode(value) end

---Converts a json string into a lua table
---@param value string
---@return table
function json:decode(value) end

lib.json = json

---Returns the specified value under the condition. **This does not evaluate!**
---@param condition boolean # The `condition` to check for
---@param true_value any # The value that will be returned if the `condition` is `true`
---@param false_value any # The value that will be returned if the `condition` is `false`
---@return any
function lib.ternary(condition, true_value, false_value) end

---Returns the specified function's value, under a condition. This will evaluate
---@param condition boolean # The `condition` to check for
---@param true_value fun(...): any # The value that will be returned if the `condition` is `true`
---@param false_value fun(...): any # The value that will be returned if the `condition` is `false`
---@return any
function lib.ternary_eval(condition, true_value, false_value, ...) end

---Converts a file into a string
---@param file_path string # The file to be converted
---@return string
function lib.file_to_string(file_path) end

--- Executes a bash command and returns it's outputs. This function will block!
---@param ... string # Command argument
function lib.run_command(...) end

return lib
