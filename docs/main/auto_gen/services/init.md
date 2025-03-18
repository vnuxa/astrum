# Table of contents

1. [`AppModel`](#appmodel) 
2. [`ApplicationsService`](#applicationsservice) 
3. [`Greetd`](#greetd) 
4. [`HyprlandClient`](#hyprlandclient) 
5. [`HyprlandWorkspace`](#hyprlandworkspace) 
6. [`MprisLoopingChanged`](#mprisloopingchanged) 
7. [`MprisOutput`](#mprisoutput) 
10. [`MprisPlayer`](#mprisplayer) 
11. [`MprisService`](#mprisservice) 
12. [`MprisShuffleToggled`](#mprisshuffletoggled) 
13. [`MprisTrackChanged`](#mpristrackchanged) 
14. [`MprisVolumeChanged`](#mprisvolumechanged) 
15. [`TimeService`](#timeservice) 
16. [`TrackMetadata`](#trackmetadata) 

[`source`](https://github.com/vnuxa/astrum/blob/unstable/src/lua_library/astrum/types/services/init.lua)

---
# AppModel
## Propreties:
>   `description` → `string`
>    >   The description of the app, if it has one 

>   `desktop` → `string`
>    >   The .desktop file path 

>   `executable` → `string`
>    >   The apps executable binary path 

>   `icon` → `string`
>    >   The icon name of the app 

>   `id` → `string`
>    >   The id of the app. The id is oobtained by the desktop file name 

>   `name` → `string`
>    >   The name of the app 

## Methods:


---
# ApplicationsService
## Propreties:
## Methods:
`:get_all_apps()` → `AppModel[]`

`:launch_app(executable_path)`
>    `executable_path`: `string`




---
# Greetd
## Propreties:
## Methods:
`:login(username, attempt, command)` → `"login_failure"|"login_success"`
>    `username`: `string`
>    >   The username to log in with 

>    `attempt`: `string`
>    >   The password you need to loog in with 

>    `command`: `string`
>    >   The command to run if login was successfull 




---
# HyprlandClient
## Propreties:
>   `at` → `{ x: number, y: number }`
>    >   The x and y position where the client is on 

>   `class` → `string`
>    >   The class name of the client 

>   `floating` → `boolean`
>    >   Is this window floating or not 

>   `initial_class` → `string`
>    >   The `initial_title` of the client 

>   `initial_title` → `string`
>    >   The `initial_title` of the client 

>   `process_id` → `number`
>    >   The process id of the client 

>   `size` → `{ x: number, y: number }`
>    >   the x and y size of the client 

>   `title` → `string`
>    >   The title of the client 

>   `workspace_id` → `number`
>    >   The id of the workspace that the client/window is on 

## Methods:


---
# HyprlandWorkspace
## Propreties:
>   `active` → `boolean`

>   `id` → `number`

## Methods:


---
# MprisLoopingChanged
## Propreties:
>   `loop_status` → `"None"|"Playlist"|"Track"`

>   `player` → `string`
>    >   The players name 

## Methods:


---
# MprisOutput
## Propreties:
>   `player` → `string`
>    >   The players name 

## Methods:


---
# MprisPlayer
## Propreties:
## Methods:
`:get_volume()` → `number`

`:next()`

`:play_pause()`

`:previous()`

`:set_loop(status)`
>    `status`: `"None"|"Playlist"|"Track"`


`:set_shuffle(shuffle)`
>    `shuffle`: `boolean`


`:set_volume(volume)`
>    `volume`: `number`




---
# MprisService
## Propreties:
## Methods:
`:get_player(player_name)` → `MprisPlayer`
>    `player_name`: `string`




---
# MprisShuffleToggled
## Propreties:
>   `player` → `string`
>    >   The players name 

>   `shuffle` → `boolean`

## Methods:


---
# MprisTrackChanged
## Propreties:
>   `player` → `string`
>    >   The players name 

>   `track` → `TrackMetadata|{ empty: boolean }`

see definitions: [`TrackMetadata`](#trackmetadata) 
## Methods:


---
# MprisVolumeChanged
## Propreties:
>   `player` → `string`
>    >   The players name 

>   `volume` → `number`

## Methods:


---
# TimeService
## Propreties:
## Methods:
`:delay(duration, signal)`
>    `duration`: `number`
>    >   How much time (in seconds) needs to pass until sending the signal 

>    `signal`: `CustomSignal`
>    >   The signal to send once the specified amount of time has passed 




---
# TrackMetadata
## Propreties:
>   `album_artists` → `string[]`
>    >   List of artists of the song 

>   `album_name` → `string`
>    >   The album name of the song playing 

>   `length` → `number`
>    >   The length of the song in seconds 

>   `title` → `string`
>    >   The title of the song playing 

## Methods:


---
