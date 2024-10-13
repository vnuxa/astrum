use cosmic::iced::futures::SinkExt;
use cosmic::iced::Subscription;
use cosmic::iced::subscription;
use system_tray::client::Client;
use system_tray::item::StatusNotifierItem;
use system_tray::menu::TrayMenu;
use system_tray::item::Category;
use system_tray::item::Status;
use std::result::Result::Ok;
use std::time::Duration;
use tokio::time::sleep;
// use simple_home_dir::home_dir;


// use std::collections::HashMap;
// use std::path::Path;
// use std::{io::{Read, Write}, os::unix::net::{UnixListener, UnixStream}};

use crate::app::WindowMessages;
use std::collections::HashMap;
use std::process;
use std::sync::MutexGuard;
// TODO: make it so that pixbufs saved and are cached somewhere in astrum
//

// TODO: make it so that you can open menus of a tray item
fn items_to_table(items: MutexGuard<HashMap<String, (StatusNotifierItem, Option<TrayMenu>)>>) -> String {
    let mut builder_string = "{ ".to_string();
    for (str_id, obj) in items.iter()  {
        let (item, menu) = obj;
        let mut builder_table = "{".to_string();
        // builder_table.push_str("\n")
        if item.title.is_some() {
            builder_table.push_str(&format!("title = '{}', ", item.title.as_ref().unwrap()));
        }
        if item.icon_name.is_some() {
            builder_table.push_str(&format!("icon_name = '{}', ", item.icon_name.as_ref().unwrap()));
        }
        if item.icon_pixmap.is_some() {
            println!("icon: {} | with pixmap!!", item.title.as_ref().unwrap());
        }
        builder_table.push_str(
            &(format!(
                "id = '{id}', category = '{category}', status = '{status}' }}, ",
                id = item.id,
                category = match item.category {
                    Category::ApplicationStatus => "application_status",
                    Category::Communications => "communications",
                    Category::Hardware => "hardware",
                    Category::SystemServices => "system_services"
                },
                status = match item.status {
                    Status::Active => "active",
                    Status::Passive => "passive",
                    Status::Unknown => "unknown",
                    Status::NeedsAttention => "needs_attention"
                }
            ))

        );

        builder_string.push_str(&builder_table);
    }
    builder_string.push_str("}");
    println!("sending: {}", builder_string);

    builder_string
}

pub fn listen_to_tray(requested_signals: HashMap<String, bool>) -> Subscription<WindowMessages> {
    let mut num = 1;

    subscription::channel("systray-listener", 100, move |mut output| async move {
        println!("systray subscription ran! {}", num);
        let client = Client::new(&format!("astrumsystray-{}", num)).await.unwrap();
        println!("wow it errored");
        num += 1;
        let mut tray_rx = client.subscribe();

        // sleep(Duration::from_secs(5)).await;
        // send a signal when subscribed so that you get the systray when launched
        if requested_signals.get("item_changed").is_some() {
            println!("making signal thingy");

            let client_items = client.items();

            sleep(Duration::from_secs(1)).await;

            output.
                try_send(WindowMessages::Msg((
                    ("systray_item_changed".to_string(), items_to_table(client_items.lock().unwrap()))
                )));
        }

        while let Ok(event) = tray_rx.recv().await {
            if requested_signals.get("item_changed").is_some() {
                println!("got request for item changing");
                // might not need update events??
                let items = client.items();
                sleep(Duration::from_secs(1)).await;
                // println!("item changed!!");

                output.
                    try_send(WindowMessages::Msg((
                        (
                            "systray_item_changed".to_string(),
                            items_to_table(items.lock().unwrap())
                        )
                    )));
            }

            // println!(" systray event: {event:?}"); // do something with event...
        }
        panic!("Systray listener failed! Exiting.");

        loop {
            cosmic::iced::futures::pending!();
        }
    })
}
