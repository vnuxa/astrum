
// allows the user to interact with their application via commands

use anyhow::Ok;
use cosmic::iced::futures::SinkExt;
use cosmic::iced::Subscription;
use cosmic::iced::subscription;
use notify::event::ModifyKind;
use notify::EventKind;
use notify::Watcher;
use simple_home_dir::home_dir;


use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;
use std::{io::{Read, Write}, os::unix::net::{UnixListener, UnixStream}};

use crate::app::WindowMessages;
use std::process;


pub fn hot_reload(dir_path: PathBuf) -> Subscription<WindowMessages>
{
    subscription::channel("astrum-reload-listener", 100, move |mut output| async move {
        let mut watcher = notify::recommended_watcher(move |result: Result<notify::Event, notify::Error>| {
            let event = result.unwrap();

            if let EventKind::Modify(modify) = event.kind {
                if let ModifyKind::Data(data) = modify {
                    output
                        .try_send(WindowMessages::ReloadLua)
                        .expect("hot-reload failed at sending a reload request");
                }
            }
        }).expect("hot-reload watcher failed");

        watcher.watch(dir_path.parent().unwrap(), notify::RecursiveMode::Recursive).expect("hot-reload config directory watcher failed");


        loop {
            cosmic::iced::futures::pending!()
        }
    })
}
