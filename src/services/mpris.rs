// mpris cannot find album art, so use another library that ags uses or just see how ags does it
// and maybe put it behind a feature???
//
// TODO: document ev erything!!! (and do this above ^ when i will be bothered to)

use std::{borrow::BorrowMut, cell::RefCell, collections::HashMap, sync::Arc, thread, time::Duration};
use hyprland::ctl::output;
use mpris::{Event, Metadata, Player, PlayerFinder};

use cosmic::iced::{advanced::graphics::{futures::MaybeSend, text::cosmic_text::ttf_parser::{math, trak::TrackData}}, futures::{channel::mpsc, stream, SinkExt, StreamExt}, subscription, Subscription};
use mpris::{DBusError, LoopStatus};
use std::sync::{Mutex, MutexGuard};

use crate::app::WindowMessages;


// use iced::futures::SinkExt;

// i cannot make it return the player iself, so the user has to get it within their view logic
// #[derive(Debug, Clone)]
pub enum PlayerListener

{
    Playing(String),
    Paused(String),
    Stopped(String),

    LoopingChanged((LoopStatus, String)),
    ShuffleToggled((bool, String)),
    VolumeChanged((f64, String)),
    TrackChanged((Metadata, String))
}


pub fn get_player_by_name(player_identity: String) -> Player {
    PlayerFinder::new()
        .expect("Could not connect to DBus")
        .find_by_name(&player_identity)
        .expect("Could not find the specified player")
}

pub fn get_first_player() -> Player {
    PlayerFinder::new()
        .expect("Could not connect to DBus")
        .find_first()
        .expect("Could not find the first player")
}


fn vec_to_table(data: Vec<&str>) -> String {
    let mut builder_string = "{".to_string();
    for (key, value) in data.iter().enumerate() {
        if key == 0{

            builder_string.push_str(&format!("'{}'", value.replace("'", r"\'")));
        } else {

            builder_string.push_str(&format!(", '{}'", value.replace("'", r"\'")));
        }
    }
    builder_string.push_str("}");
    println!("so the vec is this");
    println!("{}", builder_string);

    builder_string
}

// INFO: i dont know if more metadata is needed, but right now these are the essential
// i can think up of, if you need more let me know
fn track_metadata_to_string(track_data: Metadata) -> String {
    if track_data.is_empty() {
        return "{ empty = true }".to_string();
    }

    // maybe add track cover url
    format!(
        "{{
            album_name = '{album_name}',
            album_artists = {album_artists},
            length = {length},
            title = '{title}'

        }}",
        album_name = track_data.album_name().unwrap().replace("'", r"\'"),
        album_artists = vec_to_table(track_data.album_artists().unwrap_or(Vec::new())),
        length = (track_data.length_in_microseconds().unwrap_or(0) / 1000), // rn making it in seconds
        title = track_data.title().unwrap_or("").replace("'", r"\'"),
    )
}

pub fn listen_first_player(requested_signals: HashMap<String, bool>) -> Subscription<WindowMessages>
{

    subscription::channel("mpris-first-listener", 100, move |mut output| async move {
        tokio::task::spawn_blocking(move || {
            loop {
                let mpris_player = get_first_player();

                for event in mpris_player.events().expect("Could not start event stream").flatten() {
                    let mpris_identity = mpris_player.identity().to_string();
                    match event {
                        Event::Playing => {
                            if requested_signals.get("playing").is_some() {
                                output
                                    .try_send(
                                        WindowMessages::Msg(
                                            (
                                                "mpris_playing".to_string(),
                                                format!("{{ player = '{id}' }}", id = mpris_identity)

                                            )

                                        )
                                    )
                                    .expect("Could not send playing signal within mpris")
                            }
                        },
                        Event::Paused => {
                            if requested_signals.get("paused").is_some() {
                                output
                                    .try_send(
                                        WindowMessages::Msg(
                                            (
                                                "mpris_paused".to_string(),
                                                format!("{{ player = '{id}' }}", id = mpris_identity)

                                            )

                                        )
                                    )
                                    .expect("Could not send playing signal within mpris")
                            }
                        },
                        Event::Stopped => {
                            if requested_signals.get("stopped").is_some() {
                                output
                                    .try_send(

                                        WindowMessages::Msg(
                                            (
                                                "mpris_stopped".to_string(),
                                                format!("{{ player = '{id}' }}", id = mpris_identity)

                                            )

                                        )
                                    )
                                    .expect("Could not send playing signal within mpris")
                            }
                        },
                        Event::VolumeChanged(value) => {
                            if requested_signals.get("volume_changed").is_some() {
                                output
                                    .try_send(
                                        WindowMessages::Msg(
                                            (
                                                "mpris_volume_changed".to_string(),
                                                format!(
                                                    "{{ player = '{id}', volume = {volume} }}",
                                                    id = mpris_identity,
                                                    volume = value.to_string()
                                                )
                                            )

                                        )
                                    )
                                    .expect("Could not send playing signal within mpris")
                            }
                        },
                        Event::LoopingChanged(status) => {
                            if requested_signals.get("looping_changed").is_some() {
                                output
                                    .try_send(
                                        WindowMessages::Msg(
                                            (
                                                "mpris_looping_changed".to_string(),
                                                format!(
                                                    "{{ player = '{id}', loop_status = {loop_status} }}",
                                                    id = mpris_identity,
                                                    loop_status = match status {
                                                        LoopStatus::None => "None",
                                                        LoopStatus::Track => "Track",
                                                        LoopStatus::Playlist => "Playlist",
                                                    }
                                                )
                                            )

                                        )
                                    )
                                    .expect("Could not send playing signal within mpris")
                            }
                        },
                        Event::ShuffleToggled(value) => {
                            if requested_signals.get("shuffle_toggled").is_some() {
                                output
                                    .try_send(
                                        WindowMessages::Msg(
                                            (
                                                "mpris_shuffle_toggled".to_string(),
                                                format!(
                                                    "{{ player = '{id}', shuffle = {shuffle} }}",
                                                    id = mpris_identity,
                                                    shuffle = value.to_string()
                                                )
                                            )

                                        )
                                    )
                                    .expect("Could not send playing signal within mpris")
                            }
                        },
                        Event::TrackChanged(track_data) => {
                            if requested_signals.get("track_changed").is_some() {
                                println!("so the format is this");
                                println!("{} ", format!(
                                    "{{ player = '{id}', track = {track} }}",
                                    id = mpris_identity,
                                    track = track_metadata_to_string(track_data.clone())
                                    )
                                );

                                output
                                    .try_send(
                                        WindowMessages::Msg(
                                            (
                                                "mpris_track_changed".to_string(),
                                                format!(
                                                    "{{ player = '{id}', track = {track} }}",
                                                    id = mpris_identity,
                                                    track = track_metadata_to_string(track_data)
                                                )
                                            )

                                        )
                                    )
                                    .expect("Could not send playing signal within mpris")
                            }
                        },
                        _ => {  },
                    }

                }
            }
        }).await.unwrap();

        loop {
            cosmic::iced::futures::pending!();
        }
    })

}
//
// pub fn get_active_player() -> Player {
//     PlayerFinder::new()
//         .expect("Could not connect to DBus")
//         .find_active()
//         .expect("Could not find the first player")
// }
//
// // clone of listen_first_player but with active player instead of first
// pub fn listen_active_player() -> Subscription<PlayerListener> {
//
//     subscription::channel("mpris-active-listener", 200, |mut output| async move {
//         // if this isnt wrapped like this messages will just not be sent
//         // mainly because the loop is running so fast that messages dont have time to send
//         // probably because of blocking
//         tokio::task::spawn_blocking(move || {
//             loop {
//                 let mpris_player = get_active_player();
//                 for event in mpris_player.events().expect("Could not start event stream").flatten() {
//                     match event {
//                         Event::PlayerShutDown => break,
//                         Event::Playing  => {
//                             output
//                                 .try_send(PlayerListener::Playing)
//                                 .expect("playing within mpris failed");
//                         },
//                         Event::Paused => {
//                             output
//                                 .try_send(PlayerListener::Paused)
//                                 .expect("write error message herel ater");
//                         },
//                         Event::Stopped => {
//                             output
//                                 .try_send(PlayerListener::Stopped)
//                                 .expect("write error message herel ater");
//                         },
//
//                         Event::VolumeChanged(value) => {
//                             output
//                                 .try_send(PlayerListener::VolumeChanged(value))
//                                 .expect("write error message herel ater");
//                         },
//                         Event::LoopingChanged(status) => {
//                             output
//                                 .try_send(PlayerListener::LoopingChanged(status))
//                                 .expect("write error message herel ater");
//                         },
//
//                         Event::ShuffleToggled(value) => {
//                             output
//                                 .try_send(PlayerListener::ShuffleToggled(value))
//                                 .expect("write error message herel ater");
//                         },
//
//                         Event::TrackChanged(track_data) => {
//                             output
//                                 .try_send(PlayerListener::TrackChanged(track_data))
//                                 .expect("write error message herel ater");
//                         },
//
//                         _ => {  },
//                     };
//
//                 }
//             }
//         }).await.unwrap();
//
//         loop {
//             iced::futures::pending!();
//         }
//     })
//
// }
