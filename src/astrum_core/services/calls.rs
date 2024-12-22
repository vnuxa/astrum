
use std::{env::home_dir, io::Read, os::unix::net::UnixListener};

use cosmic::{app::Message, iced::{advanced::subscription, stream, Subscription}};

use crate::astrum_core::app::main::{AstrumMessages, StringOrNum};

pub fn calls_service_channel(requests: Vec<String>) -> Subscription<AstrumMessages> {
    Subscription::run_with_id(4, stream::channel(100, |mut output| async move {
        let socket_path = &(home_dir().unwrap().display().to_string() + "/.cache/astrum/sockets/calls");

        if std::fs::metadata(socket_path).is_ok() {
            println!("A socket is already present. Deleting...");
            std::fs::remove_file(socket_path).expect("could not delete previous socket");
        }
        let unix_listener =
            UnixListener::bind(socket_path).expect("Could not create the unix socket");

        tokio::task::spawn_blocking(move || {
            loop {
                // println!("did call check!");
                let (mut unix_stream, socket_address) = unix_listener
                    .accept() // waits for other proccesses to connect to socket
                    .expect("Failed at accepting a connection on the unix listener");

                let mut message = String::new();
                unix_stream
                    .read_to_string(&mut message)
                    .expect("failed at reading the unix listener");
                let mut end_message = (message.clone(), "{}".to_string());
                if let Some((str1, str2)) = message.split_once(" ") {
                    end_message = (str1.to_string(), str2.to_string());
                }

                if requests.contains(&end_message.0) {
                    output
                        .try_send(
                            AstrumMessages::SubscriptionRequest((
                                "calls".to_string(),
                                StringOrNum::String(end_message.0),
                                end_message.1
                            ))
                        )
                        .expect("failed to send call as a signal!")
                }

            }
        }).await.unwrap();

        loop {
            cosmic::iced_futures::futures::pending!()
        }
    }))
}
