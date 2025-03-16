local astrum = require("astrum.helper")

function astrum:application()
	local app = {
		all_messages = {},
		-- in other words, requested signals
		subscription_messages = {
			-- hyprland = {
			-- 	workspaces_changed = {
			-- 		"hello",
			-- 		"wow",
			-- 	},
			-- },
		},
		windows = {},
	}

	function app:window(window_name, window_data)
		if window_data.exclusive_zone == "ignore" then window_data.exclusive_zone = -1 end

		-- adding a list of subscrption messages with which signal they should send
		if window_data.subscriptions then
			for name, requests in pairs(window_data.subscriptions) do
				if app.subscription_messages[name] then
					-- if subscription exists, add extra signals
					for requested_signal, provided_signal in pairs(requests) do
						if app.subscription_messages[name][requested_signal] then
							table.insert(app.subscription_messages[name][requested_signal], provided_signal)
						else
							app.subscription_messages[name][requested_signal] = { provided_signal }
						end
					end
				else
					-- if a subscription doesnt exist, make one
					-- and make it so that more can be added
					app.subscription_messages[name] = {}
					for requested_signal, provided_signal in pairs(requests) do
						app.subscription_messages[name][requested_signal] = { provided_signal }
					end
				end
			end
		end
		-- add this windows signals to all of the signals in the app
		if window_data.signals then
			for name, logic in pairs(window_data.signals) do
				if type(logic) == "table" then
					for key, value in pairs(logic) do
						app.all_messages[key] = value
					end
				elseif type(logic) == "function" then
					app.all_messages[name] = logic
				end
			end
		end

		app.windows[window_name] = window_data
	end

	-- yes this might override the signal that the user is sending, but
	-- each signal is supposed to have different logic, so it should have aq different signal_name
	-- this does come with a performance benefit though during runtime and better error handling, but might be confusing for the user
	-- the last bit should be ok if it is explicitly said
	app.update_logic = function(signal_name, signal_data)
		if app.all_messages[signal_name] then
			app.all_messages[signal_name](signal_data)
		else
			print(("[WARNING]: Signal '" .. signal_name .. "' does not exist!"))
		end
	end

	app.subscription_logic = function(type, message_name, message_data)
		-- print("was requested for subscription ", type)
		if app.subscription_messages[type] then
			-- print("msg name: ", message_name)
			-- for key, value in pairs(app.subscription_messages[type]) do
			-- 	print("     the thing: ", key, " and", value)
			-- end
			if type == "keybinds" then
				app.all_messages[message_name](message_data)
				return
			end
			for _, signal_name in pairs(app.subscription_messages[type][message_name]) do
				-- print("sending a request to ", signal_name)
				app.all_messages[signal_name](message_data)
			end
		else
			print("[WARNING]: Subscription does not exist?")
		end
	end

	return app
end

return astrum
