use std::{collections::HashMap, sync::{Arc, Mutex, MutexGuard}, time::Duration};

use cosmic::iced::{futures::SinkExt, stream, Subscription};
use lazy_static::lazy_static;
use system_tray::{client::Client, item::{Category, IconPixmap, Status, StatusNotifierItem}, menu::TrayMenu};
use tokio::time::sleep;

use crate::astrum_core::app::main::{AstrumMessages, StringOrNum};

lazy_static!{
    pub static ref ICON_PIXMAPS: Arc<Mutex<HashMap<String, IconPixmap>>> = Arc::new(Mutex::new(HashMap::new()));
}


// TODO: in the future make it so that system trays menu work
fn items_to_table(items: MutexGuard<HashMap<String, (StatusNotifierItem, Option<TrayMenu>)>>) -> String {
    let mut builder_string = "{".to_string();

    for (str_id, obj) in items.iter() {
        let (item, _) = obj;
        let mut builder_table = "{".to_string();
        // println!("item: {:?}", item);

        if item.title.is_some() {
            builder_table.push_str(&format!("title = '{}', ", item.title.as_ref().unwrap()));
        }
        if item.icon_name.is_some() {
            if item.icon_name.as_ref().unwrap() != "" {
                builder_table.push_str(&format!("icon_name = '{}', ", item.icon_name.as_ref().unwrap()));
            }
        }
        if item.icon_pixmap.is_some() {
            let mut pixmaps = ICON_PIXMAPS.lock().unwrap();

            // this might have issues in the future, havent looked into pixmaps that much
            pixmaps.insert(item.id.to_string(), item.icon_pixmap.as_ref().unwrap().first().unwrap().clone());
            builder_table.push_str(&format!("icon_pixmap = '{}', ", item.id));
        }

        builder_table.push_str(
            &format!(
                "id = '{id}', category = '{category}', status = '{status}' }}, ",
                id = item.id,
                category = match item.category {
                    Category::ApplicationStatus => "application_status",
                    Category::Communications => "communications",
                    Category::SystemServices => "system_services",
                    Category::Hardware => "hardware"
                },
                status = match item.status {
                    Status::Active => "active",
                    Status::Passive => "passive",
                    Status::NeedsAttention => "needs_attention",
                    Status::Unknown => "unknown"
                }
            )
        );

        builder_string.push_str(&builder_table)
    }

    builder_string.push_str("}");

    // println!("sending: {:?}", builder_string);

    builder_string
}



pub fn system_tray_service_channel(requests: Vec<String>) -> Subscription<AstrumMessages> {
    Subscription::run_with_id(3, stream::channel(100, |mut output| async move {
        let client = Client::new("astrum_systray").await.unwrap(); // might have to add a random number/code to name?
        let mut tray_rx = client.subscribe();

        // send a signal when subscribed, so that you can get a systray on startrup
        if requests.contains(&"update".to_string()) {
            let client_items = client.items();

            sleep(Duration::from_secs(2)).await; // works well enough

            // TODO: send output of the tray items

            output
                .try_send(AstrumMessages::SubscriptionRequest(
                    (
                        "system_tray".to_string(),
                        StringOrNum::String("update".to_string()),
                        items_to_table(client_items.lock().unwrap())
                    )
                ));
        }

        while let Ok(event) = tray_rx.recv().await {
            if requests.contains(&"update".to_string()) {
                let items = client.items();

                // TODO: send output
                output
                    .try_send(AstrumMessages::SubscriptionRequest(
                        (
                            "system_tray".to_string(),
                            StringOrNum::String("update".to_string()),
                            items_to_table(items.lock().unwrap())
                        )
                    ));
            }
        }
    }))
}
