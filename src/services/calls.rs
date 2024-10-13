
// allows the user to interact with their application via commands

use anyhow::Ok;
use cosmic::iced::futures::SinkExt;
use cosmic::iced::Subscription;
use cosmic::iced::subscription;
use simple_home_dir::home_dir;


use std::collections::HashMap;
use std::path::Path;
use std::{io::{Read, Write}, os::unix::net::{UnixListener, UnixStream}};

use crate::app::WindowMessages;
use std::process;

pub fn listen_to_calls(requested_signals: HashMap<String, bool>) -> Subscription<WindowMessages>
{
    subscription::channel("astrum-calls-listener", 100, move |mut output| async move {
        // let binding = &("~/.cache/astrum/sockets/".to_owned() + &socket_name);
        // let socket_path = Path::new(binding);

        // println!("My pid is {}", process::id());

        // i dont know if i need it to be process id, right now users can only use 1 socket anyways

        // let socket_path = &(home_dir().unwrap().display().to_string() + "/.cache/astrum/sockets/" + &process::id().to_string());
        let socket_path = &(home_dir().unwrap().display().to_string() + "/.cache/astrum/sockets/calls");

        // copy-paste this and don't think about it anymore
        // it will be hidden from there on
        // println!("socket patch {}", socket_path.display());
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

                if requested_signals.get::<String>(&end_message.0).is_some() {
                    output
                        .try_send(WindowMessages::Msg(end_message))
                        .expect("failed to send call as a signal!")
                }
            }
        }).await.unwrap();

        loop {
            cosmic::iced::futures::pending!()
        }
    })
}



pub fn call_windows_socket(data: String) {

    let socket = format!("{}/.cache/astrum/sockets/windows", home_dir().unwrap().display());
    let mut unix_stream = UnixStream::connect(socket).expect("Could not create stream");

    unix_stream
        .write_all(data.as_bytes())
        .expect("failed at writing calls to window listener");

}

pub fn windows_socket(requested_signals: HashMap<String, bool>) -> Subscription<WindowMessages>
{
    subscription::channel("astrum-windows-listener", 100, move |mut output| async move {
        // let binding = &("~/.cache/astrum/sockets/".to_owned() + &socket_name);
        // let socket_path = Path::new(binding);

        println!("My pid is {}", process::id());

        // i dont know if i need it to be process id, right now users can only use 1 socket anyways

        // let socket_path = &(home_dir().unwrap().display().to_string() + "/.cache/astrum/sockets/" + &process::id().to_string());
        let socket_path = &(home_dir().unwrap().display().to_string() + "/.cache/astrum/sockets/windows");

        // copy-paste this and don't think about it anymore
        // it will be hidden from there on
        // println!("socket patch {}", socket_path.display());
        if std::fs::metadata(socket_path).is_ok() {
            println!("A socket is already present. Deleting...");
            std::fs::remove_file(socket_path).expect("could not delete previous socket");
        }
        let unix_listener =
            UnixListener::bind(socket_path).expect("Could not create the unix socket");

        tokio::task::spawn_blocking(move || {
            loop {
                let (mut unix_stream, socket_address) = unix_listener
                    .accept() // waits for other proccesses to connect to socket
                    .expect("Failed at accepting a connection on the unix listener");

                let mut message = String::new();
                unix_stream
                    .read_to_string(&mut message)
                    .expect("failed at reading the unix listener");
                if requested_signals.get::<String>(&message).is_some() {
                    output
                        .try_send(WindowMessages::ToggleWindow(message))
                        .expect("failed to send window call as a signal!")
                }
            }
        }).await.unwrap();

        loop {
            cosmic::iced::futures::pending!()
        }
    })
}
