use cosmic::iced::{advanced::subscription, stream, Subscription};
use std::time::Duration;
use mpris::{Metadata, Player, PlayerFinder};
use mpris::Event;
use mpris::LoopStatus;
use crate::astrum_core::app::main::{AstrumMessages, StringOrNum};


pub fn get_player_by_name(player_identity: &String) -> Player {
    PlayerFinder::new()
        .expect("Could not connect to dbus")
        .find_by_name(player_identity)
        .expect("Could not find the specified player")
    // might have to do an option instead of an expect
}

pub fn vec_to_table(data: Vec<&str>) -> String {
    let mut builder_string = "{".to_string();
    for (key, value) in data.iter().enumerate() {
        if key == 0{

            builder_string.push_str(&format!("'{}'", value.replace("'", r"\'")));
        } else {

            builder_string.push_str(&format!(", '{}'", value.replace("'", r"\'")));
        }
    }
    builder_string.push_str("}");

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


pub fn mpris_service_channel(requests: Vec<String>) -> Subscription<AstrumMessages> {
    Subscription::run_with_id(2, stream::channel(100, |mut output| async move {
        tokio::task::spawn_blocking(move || {

            loop {
                let mpris_player = PlayerFinder::new()
                    .expect("Could not connect to dbus")
                    .find_first();

                if let Err(err) = mpris_player {
                    // wait 500 seconds before trying to see if theres a available player
                    // might have to make the delay smaller
                    tokio::time::sleep(Duration::from_millis(500));
                    continue;
                }
                let mpris_player = mpris_player.unwrap();

                for event in mpris_player.events().expect("Could not start event stream").flatten() {
                    let mpris_identity = mpris_player.identity().to_string();

                    match event {
                        Event::Playing => {
                            if requests.contains(&"playing".to_string()) {
                                output
                                    .try_send(
                                        AstrumMessages::SubscriptionRequest(
                                            (
                                                "mpris".to_string(),
                                                StringOrNum::String("playing".to_string()),
                                                format!("{{ player = '{id}' }}", id = mpris_identity) // might
                                                // have to check if there are no ' characters that can
                                                // mess with everything
                                            )
                                        )
                                    )
                                    .expect("Could not send playing signal with mpris")
                            }
                        },
                        Event::Paused => {
                            if requests.contains(&"paused".to_string()) {
                                output
                                    .try_send(
                                        AstrumMessages::SubscriptionRequest(
                                            (
                                                "mpris".to_string(),
                                                StringOrNum::String("paused".to_string()),
                                                format!("{{ player = '{id}' }}", id = mpris_identity)
                                            )
                                        )
                                    )
                                    .expect("Could not send paused signal with mpris")
                            }
                        },
                        Event::Stopped => {
                            if requests.contains(&"stopped".to_string()) {
                                output
                                    .try_send(
                                        AstrumMessages::SubscriptionRequest(
                                            (
                                                "mpris".to_string(),
                                                StringOrNum::String("stopped".to_string()),
                                                format!("{{ player = '{id}' }}", id = mpris_identity)
                                            )
                                        )
                                    )
                                    .expect("Could not send stopped signal with mpris")
                            }
                        },
                        Event::VolumeChanged(value)=> {
                            if requests.contains(&"volume_changed".to_string()) {
                                output
                                    .try_send(
                                        AstrumMessages::SubscriptionRequest(
                                            (
                                                "mpris".to_string(),
                                                StringOrNum::String("volume_changed".to_string()),
                                                format!(
                                                    "{{ player = '{id}', volume = {volume} }}",
                                                    id = mpris_identity,
                                                    volume = value.to_string()
                                                )
                                            )
                                        )
                                    )
                                    .expect("Could not send volume changed signal with mpris")
                            }
                        },
                        Event::LoopingChanged(status) => {
                            if requests.contains(&"looping_changed".to_string()) {
                                output
                                    .try_send(
                                        AstrumMessages::SubscriptionRequest(
                                            (
                                                "mpris".to_string(),
                                                StringOrNum::String("looping_changed".to_string()),
                                                format!(
                                                    "{{ player = '{id}', loop_status = {loop_status} }}",
                                                    id = mpris_identity,
                                                    loop_status = match status {
                                                        LoopStatus::None => "none",
                                                        LoopStatus::Track => "track",
                                                        LoopStatus::Playlist => "playlist"
                                                    }
                                                )
                                            )
                                        )
                                    )
                                    .expect("Could not send looping signal with mpris")
                            }
                        }
                        Event::ShuffleToggled(value) => {
                            if requests.contains(&"shuffle_toggled".to_string()) {
                                output
                                    .try_send(
                                        AstrumMessages::SubscriptionRequest(
                                            (
                                                "mpris".to_string(),
                                                StringOrNum::String("shuffle_toggled".to_string()),
                                                format!(
                                                    "{{ player = '{id}', shuffle = {shuffle} }}",
                                                    id = mpris_identity,
                                                    shuffle = value.to_string()
                                                )
                                            )
                                        )
                                    )
                                    .expect("Could not send shuffle signal with mpris")
                            }
                        },
                        Event::TrackChanged(track_data) => {
                            if requests.contains(&"track_changed".to_string()) {
                                output
                                    .try_send(
                                        AstrumMessages::SubscriptionRequest(
                                            (
                                                "mpris".to_string(),
                                                StringOrNum::String("track_changed".to_string()),
                                                format!(
                                                    "{{ player = '{id}', track = {track} }}",
                                                    id = mpris_identity,
                                                    track = track_metadata_to_string(track_data)
                                                )
                                            )
                                        )
                                    )
                                    .expect("Could not send track changed signal with mpris")
                            }
                        }
                        _ => {  }
                    }
                }


            }
        }).await.unwrap();

        loop {
            cosmic::iced::futures::pending!();
        }
    }))
}
