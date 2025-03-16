## Signals

### `playing`
Sends a `mpris_playing` signal whenever a player started playing media\
with the following `signal_data`:
```lua
signal_data: MprisOutput

---@class (exact) MprisOutput
---@field player string # The players name
```

### `paused`
Sends a `mpris_paused` signal whenever a player was paused\
with the following `signal_data`:
```lua
signal_data: MprisOutput

---@class (exact) MprisOutput
---@field player string # The players name
```

### `stopped`
Sends a `mpris_stopped` signal whenever a player stopped\
with the following `signal_data`:
```lua
signal_data: MprisOutput

---@class (exact) MprisOutput
---@field player string # The players name
```

### `volume_changed`
Sends a `mpris_volume_changed` signal whenever a players volume was changed, the new volume is provided\
with the following `signal_data`:
```lua
signal_data: MprisVolumeChanged

---@class (exact) MprisVolumeChanged
---@field volume number
---@field player string # The players name
```
### `looping_changed`
Sends a `mpris_looping_changed` signal whenever a players loop status was changed. New loop status is provided\
with the following `signal_data`:
```lua
signal_data: MprisLoopingChanged

---@class (exact) MprisLoopingChanged
---@field player string # The players name
---@field loop_status '"None"' | '"Track"' | '"Playlist"'

```
### `shuffle_toggled`
Sends a `mpris_shuffle_toggled` signal whenever a players shuffle status was changed. New shuffle status is provided\
with the following `signal_data`:
```lua
signal_data: MprisShuffleToggled

---@class (exact) MprisShuffleToggled
---@field player string # The players name
---@field shuffle boolean

```
### `track_changed`
Sends a `mpris_track_changed` signal whenever a players track was changed. Metadata of the new track is provided\
with the following `signal_data`:
```lua
signal_data: MprisTrackChanged


---@class (exact) MprisTrackChanged: MprisOutput
---@field track TrackMetadata | { empty: boolean }

---@class (exact) MprisOutput
---@field player string # The players name

---@class (exact) TrackMetadata
---@field title string # The title of the song playing
---@field album_name string # The album name of the song playing
---@field album_artists string[] # List of artists of the song
---@field length number # The length of the song in seconds

```

---

## Methods

### `services.mpris:get_player(player_name)`
Gets a player by its name, allowing you to manage it
```lua
---@param player_name string
---@return MprisPlayer
```
MprisPlayer functions are listed as `player` below

### `player:play_pause()`
Sends a `play_pause` signal to the player

### `player:next()`
Sends a `next` signal to the player

### `player:previous()`
Sends a `previous` signal to the player

### `player:set_volume(volume)`
Sets the volume of the player
```lua
---@param volume number
```

### `player:set_loop(status)`
Sets the looping of the player
```lua
---@param status '"Playlist"' | '"Track"' | '"None"'
```

### `player:set_shuffle(shuffle)`
Sets the shuffle of the player
```lua
---@param shuffle boolean
```

### `player:get_volume()`
Gets the volume of the player
```lua
---@return number
```

