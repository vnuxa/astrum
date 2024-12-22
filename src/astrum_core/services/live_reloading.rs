
use std::{env::home_dir, io::Read, os::unix::net::UnixListener, path::PathBuf};

use cosmic::{app::Message, iced::{advanced::subscription, stream, Subscription}};
use notify::{event::ModifyKind, EventKind, Watcher};

use crate::astrum_core::app::main::{AstrumMessages, StringOrNum};

pub fn live_reload_service_channel(dir_path: PathBuf) -> Subscription<AstrumMessages> {
    Subscription::run_with_id(5, stream::channel(100, |mut output| async move {

        let mut watcher = notify::recommended_watcher(move |result: Result<notify::Event, notify::Error>| {
            let event = result.unwrap();

            if let EventKind::Modify(modify) = event.kind {
                if let ModifyKind::Data(data) = modify {
                    output
                        .try_send(AstrumMessages::LiveReload)
                        .expect("live reload failed at sending a reload request");
                }
            }
        }).expect("live reload watcher failed");

        watcher.watch(dir_path.parent().unwrap(), notify::RecursiveMode::Recursive).expect("live reload config dir watcher failed");

        loop {
            cosmic::iced_futures::futures::pending!()
        }
    }))
}
