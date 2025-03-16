# Table of contents

1. [`HyprlandSubscription`](#HyprlandSubscription) 
2. [`MprisSubscription`](#MprisSubscription) 
3. [`Notification`](#Notification) 
4. [`NotificationSubscription`](#NotificationSubscription) 
5. [`Subscriptions`](#Subscriptions) 
6. [`SystemTrayItem`](#SystemTrayItem) 
7. [`SystemTraySubscription`](#SystemTraySubscription) 

---
# HyprlandSubscription
## Propreties:
>   `active_client` → `string?`
>    >   Sends a signal whenever the active hyprland client (window) was changed (e.g. active window focus has changed). Outputs a singular `HyprlandClient` or `nil` if there is no focus 

>   `clients` → `string?`
>    >   Sends a signal whenever a hyprland client (window) was changed (for example: window closed, window opened, window moved). All of the clients ordered by their workspace is provided 

>   `workspaces` → `string?`
>    >   Sends an signal whenever hyprland workspaces change (e.g. workspace is added, workspace is moved, workspace is destroyed). All of the workspaces are provided 

## Methods:


---
# MprisSubscription
## Propreties:
>   `looping_changed` → `string?`
>    >   Sends a signal whenever the player's loop status was changed. New loop status is provided 

>   `paused` → `string?`
>    >   Sends a signal whenever the player was paused 

>   `playing` → `string?`
>    >   Sends a signal whenever the player started playing 

>   `shuffle_toggled` → `string?`
>    >   Sends a signal whenever the player's shuffle status was changed. New shuffle status is provided 

>   `stopped` → `string?`
>    >   Sends a signal whenever the player was stopped 

>   `track_changed` → `string?`
>    >   Sends a signal whenever the player's track has changed. New metadata is provided 

>   `volume_changed` → `string?`
>    >   Sends a signal whenever the player's volume was changed. New volume is provided 

## Methods:


---
# Notification
## Propreties:
>   `actions` → `string[]`
>    >   Actions are sent over as a list of pairs. Each even element in the list represents the identifier of the action. Each odd element in the list is the localized string that will be displayed to the user 

>   `app_icon` → `string`
>    >   The optional program icon of the calling application. Can be blank, indicating no icon. 

>   `app_name` → `string`
>    >   The optional name of the application sending the notification. Can be blank 

>   `body` → `string`
>    >   The optional detailed body text. Can be blank 

>   `expire_timeout` → `number`
>    >   The timeout in milliseconds since the display of the notification and at which the notification should automatically close. If it is `-1` it means that there is predefined expire timeout (up to you). If `0` then it never expires. 

>   `replaces_id` → `number`
>    >   The optional notification ID that this notification replaces. It is reccomended that the server must atomically (i.e. with no visual cues or flicker) replace the given notification with this one, so that applications can modify noitifications while they still are active. A value of 0 means that it will not replace notifications 

>   `summary` → `string`
>    >   The summary text briefly describing the notification 

## Methods:


---
# NotificationSubscription
## Propreties:
>   `on_notification` → `string`
>    >   Sends a signal whenever a notification is recieved. The `signal_data` is of type `Notification` and contains the data of the notification. 

## Methods:


---
# Subscriptions
## Propreties:
>   `hyprland` → `HyprlandSubscription?`

see definitions: [`HyprlandSubscription`](#hyprlandsubscription) 
>   `keybinds` → `table<string, [string|"alt"|"ctrl"|"shift"|"super", string|"alt"|"arrow_down"|"arrow_left"|"arrow_right"...(+34)]>?`
>    >   List of signal names (the `key` of the table) that will be sent when pressing down the specified modifiers and a character. The first parameter of the value tuple is `modifiers`, which are seperated by commas (i.e. `"shift,super"` would work when `shift` and `super` are being pressed). The 2nd parameter is a single character or lowercase name for the keybind 

>   `mpris` → `MprisSubscription?`

see definitions: [`MprisSubscription`](#mprissubscription) 
>   `notifications` → `NotificationSubscription?`

see definitions: [`Notification`](#notification) [`NotificationSubscription`](#notificationsubscription) 
>   `system_tray` → `SystemTraySubscription?`

see definitions: [`SystemTraySubscription`](#systemtraysubscription) 
>   `time` → `table<number, string>?`
>    >   Sends a signal whenever a specified amount (the `key` of the table) is passed 

## Methods:


---
# SystemTrayItem
## Propreties:
>   `category` → `"application_status"|"communications"|"hardware"|"system_services"`
>    >   The category for this item 

>   `icon_name` → `string?`
>    >   The name of the icon that should visualise the tray item 

>   `icon_pixmap` → `string?`
>    >   The id of the pixmap to use. 

>   `id` → `string`
>    >   A name that is unique for this application 

>   `status` → `"active"|"needs_attention"|"passive"|"unknown"`
>    >   Describes the status for this item or of the associated application 

>   `title` → `string?`
>    >   A name that describes the application, can be more descriptive than `id`, but it is also not nesscessary. 

## Methods:


---
# SystemTraySubscription
## Propreties:
>   `update` → `string?`
>    >   Sends a signal every time items in the tray get updated 

## Methods:


---
