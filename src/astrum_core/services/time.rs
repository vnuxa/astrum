use std::{env::home_dir, io::{Read, Write}, os::unix::net::{UnixListener, UnixStream}, thread::sleep, time::Duration};

use cosmic::iced::{stream, time, Subscription};

use crate::astrum_core::app::main::{AstrumMessages, StringOrNum};

pub fn write_delay_call(duration: mlua::Number, val: mlua::Value) {
    let socket = format!("{}/.cache/astrum/sockets/delay_call", home_dir().unwrap().display());
    let mut unix_stream = UnixStream::connect(socket).expect("Could not create stream");

    match val {
        mlua::Value::String(str) => {
            unix_stream
                .write_all(format!("{duration}|{str}", duration = duration.to_string(), str = str.to_str().unwrap()).as_bytes())
                .expect("failed at writing calls to the listener");
        },
        mlua::Value::Number(num) => {
            unix_stream
                .write_all(format!("{duration}|{num}", duration = duration.to_string(), num = num.to_string()).as_bytes())
                .expect("failed at writing calls to the listener");
        }
        mlua::Value::Table(table) => {
            let table_return = format!(
                "{duration}|{signal_name}|{signal_data}",
                duration = duration.to_string(),
                signal_name = table.get::<mlua::String>("signal_name").unwrap().to_string_lossy(),
                signal_data = table.get::<mlua::String>("signal_data").unwrap().to_string_lossy(),
            );

            unix_stream
                .write_all(table_return.as_bytes())
                .expect("failed at writing calls to the listener");
        }
        _ => unimplemented!("Delay call value not supported!")

    }

}


pub fn delay_call_service_channel() -> Subscription<AstrumMessages> {
    Subscription::run_with_id(8, stream::channel(100, |mut output| async move {
        let socket_path = &(home_dir().unwrap().display().to_string() + "/.cache/astrum/sockets/delay_call");

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


                let mut signal_name = String::new();
                unix_stream
                    .read_to_string(&mut signal_name)
                    .expect("failed at reading the unix listener");

                // first string is duration
                // second string is signal_name
                // remainder is signal_data (if provided)
                let split_signal: Vec<&str> = signal_name.splitn(3, "|").collect();
                // println!("got data!\n    data1: {data1}\n     data2: {data2}", data1 = split_signal[0], data2 = split_signal[1]);
                sleep(Duration::from_millis((split_signal[0].parse::<f32>().unwrap() * 1000.0) as u64));
                // old implementation

                if split_signal.len() == 3 {
                    output
                        .try_send(AstrumMessages::Msg(
                            (
                                split_signal[1].to_string(),
                                split_signal[2].to_string()
                            )
                        ))
                        .expect("failed to send delayed call as a signal!")
                } else {

                    output
                        .try_send(AstrumMessages::Msg(
                            (
                                split_signal[1].to_string(),
                                "{}".to_string()
                            )
                        ))
                        .expect("failed to send delayed call as a signal!")
                }

            }
        }).await.unwrap();

        loop {
            cosmic::iced::futures::pending!()
        }


    }))
}

// must be ran in release mode
pub fn time_service_channel(requested_rate: u64) -> Subscription<AstrumMessages> {
    time::every(Duration::from_secs(requested_rate)).map(move |_| {
        AstrumMessages::SubscriptionRequest(
            (
                "time".to_string(),
                StringOrNum::Num(requested_rate as i64),
                "{}".to_string(),
            )

        )
    })
}
