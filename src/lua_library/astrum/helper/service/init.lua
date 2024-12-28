local utils = require("astrum_core_utils")
local service = {
	hyprland = {},
	mpris = {},
	applications = {},
	greetd = {},
}

function service.hyprland:set_workspace(workspace) utils.hyprland_set_workspace(workspace) end
function service.hyprland:get_active_workspace() return utils.hyprland_get_active_workspace() end
function service.hyprland:get_active_client() return utils.hyprland_get_active_client() end

function service.mpris:get_player(player_name)
	local player = utils.mpris_get_player(player_name)

	function player:play_pause() player.player_play_pause() end
	function player:next() player.player_next() end
	function player:previous() player.player_previous() end

	function player:set_volume(volume) player.player_volume(volume) end
	function player:set_loop(status) player.player_looping(status) end
	function player:set_shuffle(shuffle) player.player_shuffle(shuffle) end

	function player:get_volume() return player.player_get_volume() end

	return player
end

function service.applications:get_all_apps() return utils.get_all_applications() end
function service.applications:launch_app(exec) utils.launch_application(exec) end

function service.greetd:login(username, attempt, command) utils.greetd_login(username, attempt, command) end

return service
