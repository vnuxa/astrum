// allows the user to interact with their application via commands

use cosmic::iced::futures::SinkExt;
use cosmic::iced::time;
use cosmic::iced::Subscription;
use cosmic::iced::subscription;
use simple_home_dir::home_dir;


use std::collections::HashMap;
use std::path::Path;
use std::time::Duration;
use std::{io::{Read, Write}, os::unix::net::{UnixListener, UnixStream}};

use crate::app::WindowMessages;
use std::process;

pub fn listen_to_time(requested_rate: u64) -> Subscription<WindowMessages> {
    // subscription::channel("astrum-time-listener", 100, move |mut output| async move {
    time::every(Duration::from_secs(requested_rate)).map(move |_| {
        WindowMessages::Msg(
            (
                "time_changed".to_string(),
                format!("{{ time_rate = {rate} }}", rate = requested_rate )
            )
        )
    })
    // })
}
