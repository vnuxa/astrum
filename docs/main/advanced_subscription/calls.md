# Calls
Calls is a powerful subscription that allows external proccesses to interact with astrum by sending a call signal.\
Each key in the calls subscription table will be the `call_signal` and each value will be the name of the signal that will fire when it recieved the call signal\

```lua
local called_times = 0

app:window("notification-thing", {
	view = function()
        return widgets:text("recieved call `mycall` ".. called_times.. " times")
	end,
	subscriptions = {
        calls = {
            mycall = "on_call"
        }
	},
	signals = {
        on_call = function(signal_data)
            called_times = called_times + 1
        end
	},
})
```
This will display a widget that shows the amount of times that it has recieved the `mycall` call signal\
to send this call signal, add `--call` to the options of astrum (i.e. `astrum --call mycall`)\

You can also send data through the call by adding a `:` after the call signal name, though note that it has to be in valid lua table

```lua
local data = ""

app:window("notification-thing", {
	view = function()
        return widgets:text("call `mycall` has sent data: ".. data)
	end,
	subscriptions = {
        calls = {
            mycall = "on_call"
        }
	},
	signals = {
        on_call = function(signal_data)
            data = signal_data.data
        end
	},
})
```

and executing the resulting command

```bash
astrum --call 'mycall:{data="test"}'
```

will result in the window text displaying the following
```
    call `mycall` has send data: test
```
