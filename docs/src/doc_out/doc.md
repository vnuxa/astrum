# Alignment


---

# Anchor


---

# Animation

## animate_table


```lua
(method) Animation:animate_table(false_table: table, true_table: table)
```

Animates all of the tables number values, based on the animation's state

@*param* `false_table` — The table to go to when the `animation state` is `false`

@*param* `true_table` — The table to go to when the `animation state` is `true`

## animate_value


```lua
(method) Animation:animate_value(false_value: number, true_value: number)
```

Animates any number value, based on the animation state

@*param* `false_value` — The value to transition to when the `animation state` is `false`

@*param* `true_value` — The value to transition to when the `animation state` is `true`

## animation_id


```lua
number
```

The animation's ID, used for interracting with lilt animations

## get_state


```lua
(method) Animation:get_state()
  -> boolean
```

Gets the current animation state

## play


```lua
(method) Animation:play()
```

Plays an animation  and toggles the state

## toggle


```lua
(method) Animation:toggle(state?: boolean)
```

Changes the animation state without playing an animation

@*param* `state` — If provided, change to a specific state


---

# Animations

## animate_style


```lua
(method) Animations:animate_style(animation: Animation, false_style: string, true_style: string)
```

Animates a widget's style, based on the animation's state. Both provided styles must have a number value in order to animate it

@*param* `false_style` — The classname of the requested style to go to when the `animation` is `false`.

@*param* `true_style` — The classname of the requested style to go to when the `animation` is `true`.

## new


```lua
(method) Animations:new(starting_value: boolean|nil, time: number|nil, easing: "ease_in"|"ease_in_back"|"ease_in_bounce"|"ease_in_circ"|"ease_in_cubic"...(+27), repeat_amount: number|nil, reverse: boolean|nil, delay: number|nil)
  -> Animation
```

Makes a new animation based on the provided settings

@*param* `starting_value` — The state of the animation, default is `false`

@*param* `time` — The amount of seconds it takes for the animation to complete. Default is `1`

@*param* `easing` — The easing style of the animation, default is `"linear"`

@*param* `repeat_amount` — The amount of times the animation will repeat itself, default is `nil`

@*param* `reverse` — Will the animation reverse itself, default is `false`

@*param* `delay` — The delay until the animation will start, in seconds. Default is `0`

```lua
easing:
    | "linear"
    | "ease_in"
    | "ease_out"
    | "ease_in_out"
    | "ease_in_quad"
    | "ease_out_quad"
    | "ease_in_out_quad"
    | "ease_in_cubic"
    | "ease_out_cubic"
    | "ease_in_out_cubic"
    | "ease_in_quart"
    | "ease_out_quart"
    | "ease_in_out_quart"
    | "ease_in_quint"
    | "ease_out_quint"
    | "ease_in_out_quint"
    | "ease_in_expo"
    | "ease_out_expo"
    | "ease_in_out_expo"
    | "ease_in_circ"
    | "ease_out_circ"
    | "ease_in_out_circ"
    | "ease_in_back"
    | "ease_out_back"
    | "ease_in_out_back"
    | "ease_in_elastic"
    | "ease_out_elastic"
    | "ease_in_out_elastic"
    | "ease_in_bounce"
    | "ease_out_bounce"
    | "ease_in_out_bounce"
```


---

# AppModel

## description


```lua
string
```

The description of the app, if it has one

## desktop


```lua
string
```

The .desktop file path

## executable


```lua
string
```

The apps executable binary path

## icon


```lua
string
```

The icon name of the app

## id


```lua
string
```

The id of the app. The id is oobtained by the desktop file name

## name


```lua
string
```

The name of the app


---

# ApplicationsService

## get_all_apps


```lua
(method) ApplicationsService:get_all_apps()
  -> AppModel[]
```

Gets all applications within .desktop

## launch_app


```lua
(method) ApplicationsService:launch_app(executable_path: string)
```

Launches an application, using the executable path


---

# Astrum

## application


```lua
(method) Astrum:application()
  -> AstrumApp
```

Makes an astrum application that is to be returned


---

# AstrumApp

## window


```lua
(method) AstrumApp:window(window_name: string, window_model: WindowModel)
```

Creates a toplevel surface, that holds and processes signals and widgets

@*param* `window_name` — The unique name of the window

@*param* `window_model` — The model of the window


---

# ButtonModel

## child


```lua
Widget?
```

A widget that will be displayed within the button

## height


```lua
("fill"|"shrink"|["fill_portion", number]|["fixed", number])?
```

Sets the height of the widget

## on_press


```lua
(string|CustomSignal)?
```

Sends a signal whenever the button is pressed. If a string is provided, it will send no data

## padding


```lua
(number|[number, number, number, number]|[number, number])?
```

## width


```lua
("fill"|"shrink"|["fill_portion", number]|["fixed", number])?
```

Sets the width of the widget


---

# CenterboxModel

## align_items


```lua
("center"|"end"|"start")?
```

## height


```lua
("fill"|"shrink"|["fill_portion", number]|["fixed", number])?
```

Sets the height of the widget

## left_child


```lua
Widget?
```

Element to be displayed on the left side of the centerbox

## middle_child


```lua
Widget?
```

Element to be displayed in the middle of the centerbox

## padding


```lua
(number|[number, number, number, number]|[number, number])?
```

## right_child


```lua
Widget?
```

Element to be displayed on the right side of the centerbox

## spacing


```lua
number?
```

The spacing of elements in pixels

## width


```lua
("fill"|"shrink"|["fill_portion", number]|["fixed", number])?
```

Sets the width of the widget


---

# ColumnModel

## align_x


```lua
("center"|"left"|"right")?
```

Sets the vertical alignments of the contents of the `column`

## children


```lua
Widget[]?
```

List of widgets to be rendered within the `column`

## clip


```lua
boolean?
```

Sets whether the contents of the `column` should be clipped on overflow

## height


```lua
("fill"|"shrink"|["fill_portion", number]|["fixed", number])?
```

Sets the height of the widget

## max_width


```lua
number?
```

Maximum width of the `column` in pixels

## padding


```lua
(number|[number, number, number, number]|[number, number])?
```

## spacing


```lua
number?
```

The spacing between elements in pixels

## width


```lua
("fill"|"shrink"|["fill_portion", number]|["fixed", number])?
```

Sets the width of the widget


---

# ContainerModel

## align_x


```lua
("center"|"left"|"right")?
```

Sets the alignment of content on the horizontal axis

## align_y


```lua
("bottom"|"center"|"top")?
```

Sets the alignment of content on the vertical axis

## center_x


```lua
("fill"|"shrink"|["fill_portion", number]|["fixed", number])?
```

Sets the width of the `container` and centers the content horizontally

## center_y


```lua
("fill"|"shrink"|["fill_portion", number]|["fixed", number])?
```

Sets the height of the `container` and centers the content vertically

## child


```lua
Widget?
```

Element to be displayed within the `container`

## height


```lua
("fill"|"shrink"|["fill_portion", number]|["fixed", number])?
```

Sets the height of the widget

## max_height


```lua
number?
```

Maximum height of the `container` in pixels

## max_width


```lua
number?
```

Maximum width of the `container` in pixels

## padding


```lua
(number|[number, number, number, number]|[number, number])?
```

## width


```lua
("fill"|"shrink"|["fill_portion", number]|["fixed", number])?
```

Sets the width of the widget


---

# ContentFit


---

# CustomSignal

## signal_data


```lua
table
```

## signal_name


```lua
string
```


---

# EasingStyle


---

# FilterMethod


---

# Font

## name


```lua
string
```

The font's name

## style


```lua
("italic"|"normal"|"oblique")?
```

The style of the font

## weight


```lua
("black"|"bold"|"extrabold"|"extralight"|"light"...(+4))?
```

The font's weight


---

# FontStyle


---

# FontWeight


---

# Greetd

## login


```lua
(method) Greetd:login(username: string, attempt: string, command: string)
  -> "login_failure"|"login_success"
```

Attempts to login via greetd

@*param* `username` — The username to log in with

@*param* `attempt` — The password you need to loog in with

@*param* `command` — The command to run if login was successfull

```lua
return #1:
    | "login_success"
    | "login_failure"
```


---

# Horizontal


---

# HyprlandClient

## at


```lua
{ x: number, y: number }
```

The x and y position where the client is on

## class


```lua
string
```

The class name of the client

## floating


```lua
boolean
```

Is this window floating or not

## initial_class


```lua
string
```

The `initial_title` of the client

## initial_title


```lua
string
```

The `initial_title` of the client

## process_id


```lua
number
```

The process id of the client

## size


```lua
{ x: number, y: number }
```

the x and y size of the client

## title


```lua
string
```

The title of the client

## workspace_id


```lua
number
```

The id of the workspace that the client/window is on


---

# HyprlandClients


---

# HyprlandSubscription

## active_client


```lua
string?
```

Sends a signal whenever the active hyprland client (window) was changed (e.g. active window focus has changed). Outputs a singular `HyprlandClient` or `nil` if there is no focus

## clients


```lua
string?
```

Sends a signal whenever a hyprland client (window) was changed (for example: window closed, window opened, window moved). All of the clients ordered by their workspace is provided

## workspaces


```lua
string?
```

Sends an signal whenever hyprland workspaces change (e.g. workspace is added, workspace is moved, workspace is destroyed). All of the workspaces are provided


---

# HyprlandWorkspace

## active


```lua
boolean
```

## id


```lua
number
```


---

# HyprlandWorkspaces


---

# IconModel

## content_fit


```lua
("contain"|"cover"|"fill"|"none"|"scale_down")?
```

Sets how the content should be fit.

## height


```lua
("fill"|"shrink"|["fill_portion", number]|["fixed", number])?
```

Sets the height of the widget

## icon_name


```lua
string?
```

The name of the icon. `icon_name` or `icon_path` is required.

## icon_path


```lua
string?
```

The path to the icon. `icon_name` or `icon_path` is required.

## size


```lua
integer?
```

The size of the icon.

## width


```lua
("fill"|"shrink"|["fill_portion", number]|["fixed", number])?
```

Sets the width of the widget


---

# ImageModel

## border_radius


```lua
(number|[number, number, number, number])?
```

The border radius of the image

## content


```lua
string?
```

A path to an image, this field is required

## content_fit


```lua
("contain"|"cover"|"fill"|"none"|"scale_down")?
```

Sets how the content should be fit. Defaults to `contain`

## filter_method


```lua
("linear"|"nearest")?
```

## height


```lua
("fill"|"shrink"|["fill_portion", number]|["fixed", number])?
```

Sets the height of the widget

## opacity


```lua
number?
```

Sets the opacity of an image. It should be in `0.0 - 1.0` range

## rotation


```lua
(["floating", number]|["solid", number])?
```

Sets the rotation of the image. `floating` - element will float while rotating, layout will be the same prior to rotation (default). `solid` - element will be solid while rotating, layout will be adjusted to fit rotated content

## width


```lua
("fill"|"shrink"|["fill_portion", number]|["fixed", number])?
```

Sets the width of the widget


---

# Keymode


---

# Layer


---

# Length


---

# LineHeight


---

# MatchUtil

## arm


```lua
fun(condition: any, result: any):MatchUtil
```

Checks if the `scrutinee` is equal to the condition (`==`)

## arm_less_eq


```lua
fun(condition: any, result: any):MatchUtil
```

Checks if the `scrutinee` is less than or equal to the condition (`<=`)

## arm_less_than


```lua
fun(condition: any, result: any):MatchUtil
```

Checks if the `scrutinee` is less than to the condition (`<`)

## arm_more_eq


```lua
fun(condition: any, result: any):MatchUtil
```

Checks if the `scrutinee` is more than or equal to the condition (`>=`)

## arm_more_than


```lua
fun(condition: any, result: any):MatchUtil
```

Checks if the `scrutinee` is more than to the condition (`>`)

## collapse


```lua
fun():any
```

Collapses the entire match arm, returning either a result, the fallback if no match was found or nothing

## default


```lua
fun(result: any):MatchUtil
```

If none of the arms have a match, it will fallback to this result


---

# Modifier


---

# MprisLoopingChanged

## loop_status


```lua
"None"|"Playlist"|"Track"
```

## player


```lua
string
```

The players name


---

# MprisOutput

## player


```lua
string
```

The players name


---

# MprisPlayer

## get_volume


```lua
(method) MprisPlayer:get_volume()
  -> number
```

Gets the volume of the player

## next


```lua
(method) MprisPlayer:next()
```

Sends a `next` signal to the player

## play_pause


```lua
(method) MprisPlayer:play_pause()
```

Sends a `play_pause` signal to the player

## previous


```lua
(method) MprisPlayer:previous()
```

Sends a `previous` signal to the player

## set_loop


```lua
(method) MprisPlayer:set_loop(status: "None"|"Playlist"|"Track")
```

Sets the looping of the player

```lua
status:
    | "Playlist"
    | "Track"
    | "None"
```

## set_shuffle


```lua
(method) MprisPlayer:set_shuffle(shuffle: boolean)
```

Sets the shuffle of the player

## set_volume


```lua
(method) MprisPlayer:set_volume(volume: number)
```

Sets the volume of the player


---

# MprisService

## get_player


```lua
(method) MprisService:get_player(player_name: string)
  -> MprisPlayer
```

Gets a player by its name, allowing you to manage it


---

# MprisShuffleToggled

## player


```lua
string
```

The players name

## shuffle


```lua
boolean
```


---

# MprisSubscription

## looping_changed


```lua
string?
```

Sends a signal whenever the player's loop status was changed. New loop status is provided

## paused


```lua
string?
```

Sends a signal whenever the player was paused

## playing


```lua
string?
```

Sends a signal whenever the player started playing

## shuffle_toggled


```lua
string?
```

Sends a signal whenever the player's shuffle status was changed. New shuffle status is provided

## stopped


```lua
string?
```

Sends a signal whenever the player was stopped

## track_changed


```lua
string?
```

Sends a signal whenever the player's track has changed. New metadata is provided

## volume_changed


```lua
string?
```

Sends a signal whenever the player's volume was changed. New volume is provided


---

# MprisTrackChanged

## player


```lua
string
```

The players name

## track


```lua
TrackMetadata|{ empty: boolean }
```


---

# MprisVolumeChanged

## player


```lua
string
```

The players name

## volume


```lua
number
```


---

# NamedKeys


---

# Notification

## actions


```lua
string[]
```

Actions are sent over as a list of pairs. Each even element in the list represents the identifier of the action. Each odd element in the list is the localized string that will be displayed to the user

## app_icon


```lua
string
```

The optional program icon of the calling application. Can be blank, indicating no icon.

## app_name


```lua
string
```

The optional name of the application sending the notification. Can be blank

## body


```lua
string
```

The optional detailed body text. Can be blank

## expire_timeout


```lua
number
```

The timeout in milliseconds since the display of the notification and at which the notification should automatically close. If it is `-1` it means that there is predefined expire timeout (up to you). If `0` then it never expires.

## replaces_id


```lua
number
```

The optional notification ID that this notification replaces. It is reccomended that the server must atomically (i.e. with no visual cues or flicker) replace the given notification with this one, so that applications can modify noitifications while they still are active. A value of 0 means that it will not replace notifications

## summary


```lua
string
```

The summary text briefly describing the notification


---

# NotificationSubscription

## on_notification


```lua
string
```

Sends a signal whenever a notification is recieved. The `signal_data` is of type `Notification` and contains the data of the notification.


---

# Padding


---

# Proto


---

# RowModel

## align_y


```lua
("bottom"|"center"|"top")?
```

Sets the vertical alignments of the contents of the `row`

## children


```lua
Widget[]?
```

List of widgets to be rendered within the `row`

## clip


```lua
boolean?
```

Sets whether the contents of the `row` should be clipped on overflow

## height


```lua
("fill"|"shrink"|["fill_portion", number]|["fixed", number])?
```

Sets the height of the widget

## padding


```lua
(number|[number, number, number, number]|[number, number])?
```

## spacing


```lua
number?
```

The spacing between elements in pixels

## width


```lua
("fill"|"shrink"|["fill_portion", number]|["fixed", number])?
```

Sets the width of the widget


---

# Subscriptions

## calls


```lua
table<string, string>?
```

Sends to a signal when the specified call name (in the index) has been sent

## hyprland


```lua
HyprlandSubscription?
```

## keybinds


```lua
table<string, [string|"alt"|"ctrl"|"shift"|"super", string|"alt"|"arrow_down"|"arrow_left"|"arrow_right"...(+34)]>?
```

List of signal names (the `key` of the table) that will be sent when pressing down the specified modifiers and a character. The first parameter of the value tuple is `modifiers`, which are seperated by commas (i.e. `"shift,super"` would work when `shift` and `super` are being pressed). The 2nd parameter is a single character or lowercase name for the keybind

## mpris


```lua
MprisSubscription?
```

## notifications


```lua
NotificationSubscription?
```

## system_tray


```lua
SystemTraySubscription?
```

## time


```lua
table<number, string>?
```

Sends a signal whenever a specified amount (the `key` of the table) is passed


---

# SwitchUtil

## case


```lua
fun(condition: any, result: fun():any):SwitchUtil
```

Checks if the `scrutinee` is equal to the condition (`==`)

## case_less_eq


```lua
fun(condition: any, result: fun():any):SwitchUtil
```

Checks if the `scrutinee` is less than or equal to the condition (`<=`)

## case_less_than


```lua
fun(condition: any, result: fun():any):SwitchUtil
```

Checks if the `scrutinee` is less than to the condition (`<`)

## case_more_eq


```lua
fun(condition: any, result: fun():any):SwitchUtil
```

Checks if the `scrutinee` is more than or equal to the condition (`>=`)

## case_more_than


```lua
fun(condition: any, result: fun():any):SwitchUtil
```

Checks if the `scrutinee` is more than to the condition (`>`)

## collapse


```lua
fun()
```

Evaluates the found case, if no case was found then evaluate the fallback if it is provided

## default


```lua
fun(result: fun():any):MatchUtil
```

If none of the arms have a match, it will fallback to this result


---

# SystemTrayItem

## category


```lua
"application_status"|"communications"|"hardware"|"system_services"
```

The category for this item

## icon_name


```lua
string?
```

The name of the icon that should visualise the tray item

## icon_pixmap


```lua
string?
```

The id of the pixmap to use.

## id


```lua
string
```

A name that is unique for this application

## status


```lua
"active"|"needs_attention"|"passive"|"unknown"
```

Describes the status for this item or of the associated application

## title


```lua
string?
```

A name that describes the application, can be more descriptive than `id`, but it is also not nesscessary.


---

# SystemTraySubscription

## update


```lua
string?
```

Sends a signal every time items in the tray get updated


---

# TextInputModel

## always_active


```lua
boolean?
```

If enabled, makes it behave as if it were always focused

## line_height


```lua
(["absolute", number]|["relative", number])?
```

Sets the line height of the

## on_input


```lua
string?
```

Runs a signal when some text is typed in the text input, sends `text` in the signal data which contains the new text. Cannot pass through custom signals

## on_submit


```lua
(string|CustomSignal)?
```

Sends a custom signal when the text input is focused and the enter key is pressed

## password


```lua
boolean?
```

If the text input should be a secure password input

## placeholder


```lua
string?
```

Placeholder text for the text input

## size


```lua
number?
```

Sets the text size of the text input

## value


```lua
string?
```

The text of the text input. Needs an external variable paired with `on_input` in order to change

## width


```lua
("fill"|"shrink"|["fill_portion", number]|["fixed", number])?
```


---

# TextModel

## content


```lua
string?
```

The text that should be displayed

## font


```lua
Font?
```

The font of the text

## height


```lua
("fill"|"shrink"|["fill_portion", number]|["fixed", number])?
```

Sets the height of the widget

## size


```lua
number?
```

The font size of the text

## width


```lua
("fill"|"shrink"|["fill_portion", number]|["fixed", number])?
```

Sets the width of the widget


---

# TimeService

## delay


```lua
(method) TimeService:delay(duration: number, signal: CustomSignal)
```

Sends a signal after a specificed amount of time

@*param* `duration` — How much time (in seconds) needs to pass until sending the signal

@*param* `signal` — The signal to send once the specified amount of time has passed


---

# Trace


---

# TrackMetadata

## album_artists


```lua
string[]
```

List of artists of the song

## album_name


```lua
string
```

The album name of the song playing

## length


```lua
number
```

The length of the song in seconds

## title


```lua
string
```

The title of the song playing


---

# TrayCategory


---

# TrayStatus


---

# Vertical


---

# Widget

## widget_name


```lua
string
```


---

# Widgets

## button


```lua
(method) Widgets:button(content_or_model?: string|ButtonModel, extra_model?: ButtonModel)
  -> Widget
```

A container that emits a signal when pressed on

@*param* `content_or_model` — The text to be displayed (shorthand) or the model for the button

@*param* `extra_model` — The params for the shorthand. You do not need this if you provide a model in the first argument

## centerbox


```lua
(method) Widgets:centerbox(model?: CenterboxModel)
  -> Widget
```

A container that can distribute content on the left, middle and right sides

## column


```lua
(method) Widgets:column(model?: ColumnModel)
  -> table
```

A container that distributes content horizontally

## container


```lua
(method) Widgets:container(model: ContainerModel)
  -> Widget
```

 A container that can be decorated or used for alignment

## icon


```lua
(method) Widgets:icon(icon_name_or_model?: string|IconModel, extra_model?: IconModel)
  -> Widget
```

 A lazily generated, generic icon

@*param* `icon_name_or_model` — The icon name (shorthand) or the model for the icon

@*param* `extra_model` — Extra params for the shorthand. You do not need this if you already provided a model in the first argument

## image


```lua
(method) Widgets:image(content_or_model?: string|ImageModel, extra_model?: ImageModel)
  -> Widget
```

 A frame that displays an iamge while keeping a speciifed aspect ratio

@*param* `content_or_model` — The text to be displayed (shorthand) or the model for the image

@*param* `extra_model` — The params for the shorthand. You do not need this if you provide a model in the first argument

## row


```lua
(method) Widgets:row(model?: RowModel)
  -> table
```

A container that distributes content horizontally

## signal


```lua
(method) Widgets:signal(signal_name: string, signal_data: table)
  -> CustomSignal
```

Creates a signal that can send in custom data

@*param* `signal_data` — Data to be sent through the signal

## text


```lua
(method) Widgets:text(content_or_model?: string|TextModel, extra_model?: TextModel)
  -> Widget
```

A widget that holds text

@*param* `content_or_model` — The text to be displayed (shorthand) or the model for the text

@*param* `extra_model` — Extra params for the shorthand. You do not need this if you provide a model in the first argument

## text_input


```lua
(method) Widgets:text_input(content_or_model: string|TextInputModel, placeholder?: string, model?: TextInputModel)
  -> Widget
```

 A field which can be filled with text.


---

# WidthHeightWidget

## height


```lua
("fill"|"shrink"|["fill_portion", number]|["fixed", number])?
```

Sets the height of the widget

## width


```lua
("fill"|"shrink"|["fill_portion", number]|["fixed", number])?
```

Sets the width of the widget


---

# WindowModel

## anchor


```lua
"bottom"|"left"|"right"|"top"[]?
```

## exclusive_zone


```lua
(integer|"ignore")?
```

How much space should the window reserve, set it to `"ignore"` if you want it to ignore other layers

## is_popup


```lua
boolean?
```

## keymode


```lua
("exclusive"|"none"|"on_demand")?
```

## layer


```lua
("background"|"bottom"|"top")?
```

## signals


```lua
({ [string]: fun(signal_data: table) }|{ [string]: fun(signal_data: table) }[])?
```

A dictionary of signal names and their respective logic that will be processed when the signal is called on. If a table value is provided, it will unpack it. If there are multiple signals with the same name, it will get overriden

## subscriptions


```lua
Subscriptions?
```

Connects to an external processes by sending signals. All of the subscriptions are to be provided in a table

## view


```lua
fun():Widget
```

Logic that dictates what widgets for the window to render


---

# WindowSignal


---

# jit.funcinfo.c

## ffid


```lua
integer|nil
```


---

# jit.funcinfo.lua


---

# jit.snap


---

# jit.traceinfo