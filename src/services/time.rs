// allows the user to interact with their application via commands

use cosmic::iced::futures::SinkExt;
use cosmic::iced::time;
use cosmic::iced::Subscription;
use cosmic::iced::subscription;
use simple_home_dir::home_dir;


use std::collections::HashMap;
use std::f32;
use std::path::Path;
use std::slice::SplitInclusive;
use std::thread::sleep;
use std::time::Duration;
use std::{io::{Read, Write}, os::unix::net::{UnixListener, UnixStream}};

use crate::app::WindowMessages;
use std::process;

pub fn write_delay_call(duration: mlua::Number, val: mlua::Value) {
    let socket = format!("{}/.cache/astrum/sockets/delay_call", home_dir().unwrap().display());
    let mut unix_stream = UnixStream::connect(socket).expect("Could not create stream");

    match val {
        mlua::Value::String(str) => {
            unix_stream
                .write_all(format!("{duration}|{str}", duration = duration.to_string(), str = str.to_str().unwrap()).as_bytes())
                .expect("failed at writing calls to the listener");
        },
        mlua::Value::Table(table) => {
            let table_return = format!(
                "{duration}|{signal_name}|{signal_data}",
                duration = duration.to_string(),
                signal_name = table.get::<_, mlua::String>("signal_name").unwrap().to_str().unwrap(),
                signal_data = table.get::<_, mlua::String>("signal_data").unwrap().to_str().unwrap(),
            );

            unix_stream
                .write_all(table_return.as_bytes())
                .expect("failed at writing calls to the listener");
        }
        _ => unimplemented!("Delay call value not supported!")
    }

}

pub fn add_delay_calls() -> Subscription<WindowMessages> {
    subscription::channel("astrum-delay-calls-listener", 100, move |mut output| async move {
        let socket_path = &(home_dir().unwrap().display().to_string() + "/.cache/astrum/sockets/delay_call");

        if std::fs::metadata(socket_path).is_ok() {
            println!("A socket is already present. Deleting...");
            std::fs::remove_file(socket_path).expect("could not delete previous socket");
        }
        let unix_listener =
            UnixListener::bind(socket_path).expect("Could not create the unix socket");

        tokio::task::spawn_blocking(move || {
            loop {
                // println!("did delay call check!");
                let (mut unix_stream, socket_address) = unix_listener
                    .accept() // waits for other proccesses to connect to socket
                    .expect("Failed at accepting a connection on the unix listener");

                let mut signal_name = String::new();
                unix_stream
                    .read_to_string(&mut signal_name)
                    .expect("failed at reading the unix listener");

                // first string is duration
                // second string is signal_name
                // remainder is signal_data (if provided)
                // println!("signal before: {}", signal_name);
                let split_signal: Vec<&str> = signal_name.splitn(3, "|").collect();
                // println!("got data! {:?}", split_signal);
                println!("got data!\n    data1: {data1}\n     data2: {data2}", data1 = split_signal[0], data2 = split_signal[1]);
                // \n    data3: {data3}",data1 = split_signal[0], data2 = split_signal[1], data3 = split_signal[2]);
                sleep(Duration::from_millis((split_signal[0].parse::<f32>().unwrap() * 1000.0) as u64));
                // sleep(Duration::from_secs(split_signal[0].parse().unwrap())); // trhis is the
                // old implementation

                if split_signal.len() == 3 {
                    output
                        .try_send(WindowMessages::Msg(
                            (
                                split_signal[1].to_string(),
                                split_signal[2].to_string()
                            )
                        ))
                        .expect("failed to send delayed call as a signal!")
                } else {

                    output
                        .try_send(WindowMessages::Msg(
                            (
                                split_signal[1].to_string(),
                                "{}".to_string()
                            )
                        ))
                        .expect("failed to send delayed call as a signal!")
                }


                // if let Some((str1, str2, remainder)) = signal_name.splitn(2, "| ") {

                //     output
                //         .try_send(WindowMessages::Msg(
                //             (
                //                 str1.to_string(),
                //                 str2.to_string()
                //             )
                //         ))
                //         .expect("failed to send delayed call as a signal!")
                // } else {
                //     output
                //         .try_send(WindowMessages::Msg(
                //             (
                //                 signal_name,
                //                 "{}".to_string()
                //             )
                //         ))
                //         .expect("failed to send delayed call as a signal!")
                // }
            }
        }).await.unwrap();

        loop {
            cosmic::iced::futures::pending!()
        }
    })
}

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
