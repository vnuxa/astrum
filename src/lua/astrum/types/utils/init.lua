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

---@class MatchUtil
---@field arm fun(condition: any, result: any): MatchUtil # Checks if the `scrutinee` is equal to the condition (`==`)
---@field arm_less_eq fun(condition: any, result: any): MatchUtil # Checks if the `scrutinee` is less than or equal to the condition (`<=`)
---@field arm_more_eq fun(condition: any, result: any): MatchUtil # Checks if the `scrutinee` is more than or equal to the condition (`>=`)
---@field arm_less_than fun(condition: any, result: any): MatchUtil # Checks if the `scrutinee` is less than to the condition (`<`)
---@field arm_more_than fun(condition: any, result: any): MatchUtil # Checks if the `scrutinee` is more than to the condition (`>`)
---@field default fun(result: any): MatchUtil # If none of the arms have a match, it will fallback to this result
---@field collapse fun(): any # Collapses the entire match arm, returning either a result, the fallback if no match was found or nothing

---Matches the `scrutinee` with all of the specified `arm conditions`. Match does not evaluate and only returns, if you need something that only needs to evaluate use `switch`
---@param scrutinee any # The variable to match all of your arms to
---@return MatchUtil
function lib.match(scrutinee) end

---@class SwitchUtil
---@field case fun(condition: any, result: fun(): any): SwitchUtil # Checks if the `scrutinee` is equal to the condition (`==`)
---@field case_less_eq fun(condition: any, result: fun(): any): SwitchUtil # Checks if the `scrutinee` is less than or equal to the condition (`<=`)
---@field case_more_eq fun(condition: any, result: fun(): any): SwitchUtil # Checks if the `scrutinee` is more than or equal to the condition (`>=`)
---@field case_less_than fun(condition: any, result: fun(): any): SwitchUtil # Checks if the `scrutinee` is less than to the condition (`<`)
---@field case_more_than fun(condition: any, result: fun(): any): SwitchUtil # Checks if the `scrutinee` is more than to the condition (`>`)
---@field default fun(result: fun(): any): MatchUtil # If none of the arms have a match, it will fallback to this result
---@field collapse fun() # Evaluates the found case, if no case was found then evaluate the fallback if it is provided

---Matches the `scrutinee` with all of the specified `cases`. Switch does not return anything and will only evaluate a specified case.
---@param scrutinee any # The variable to match all of your cases to
---@return SwitchUtil
function lib.switch(scrutinee) end

---Converts a file into a string
---@param file_path string # The file to be converted
---@return string
function lib.file_to_string(file_path) end

--- Executes a bash command and returns it's outputs. This function will block!
---@param ... string # Command argument
function lib.run_command(...) end

return lib
