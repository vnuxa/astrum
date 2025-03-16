use std::collections::HashMap;

use cosmic::iced::{stream, Subscription};
use tokio::sync::mpsc::Sender;
use zbus::{interface, proxy, Connection};

use crate::astrum_core::app::main::{AstrumMessages, StringOrNum};

use super::mpris::vec_to_table;


pub struct NotificationTest
    {
        pub callback: Box<dyn FnMut(
            &str, // app_name
            u32, // replaces_id
            &str, // app_icon
            &str, // summary
            &str, // body
            Vec<&str>, // actions
            // HashMap<&str, zbus::zvariant::Value<'_>>, // hints
            i32, // expire_timeout
        ) + Send + Sync>
    }

#[interface(name = "org.freedesktop.Notifications")]
impl NotificationTest {
    #[zbus(out_args("name", "vendor", "version", "spec_version"))]
    async fn get_server_information(
        &self,
    ) -> (&'static str, &'static str, &'static str, &'static str) {
        ("astrum", "vnuxa", env!("CARGO_PKG_VERSION"), "1.2")
    }

    /// Notify method
    async fn notify(
        &mut self,
        app_name: &str,
        replaces_id: u32,
        app_icon: &str,
        summary: &str,
        body: &str,
        actions: Vec<&str>,
        hints: HashMap<&str, zbus::zvariant::Value<'_>>,
        expire_timeout: i32,
    )  -> u32 {
        (self.callback)(app_name, replaces_id, app_icon, summary, body, actions, expire_timeout);
        // println!("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!! notification thing was called app name: {:?} | replaced id: {:?} | app_icon: {:?} | summary: {:?}", app_name, replaces_id, app_icon, summary);
        0
    }

}



pub fn notifications_service_channel() -> Subscription<AstrumMessages> {
    Subscription::run_with_id(7, stream::channel(100, |mut output| async move {
        let test = NotificationTest {
            callback: Box::new(move |app_name: &str, replaces_id: u32, app_icon: &str, summary: &str, body: &str, actions: Vec<&str>, expire_timeout: i32| {
                output
                    .try_send(
                        AstrumMessages::SubscriptionRequest(
                            (
                                "notifications".to_string(),
                                StringOrNum::String("on_notification".to_string()),
                                format!(
                                    "{{
                                        app_name = '{}',
                                        replaces_id = '{}',
                                        app_icon = '{}',
                                        summary = '{}',
                                        body = '{}',
                                        actions = '{}',
                                        expire_timeout = '{}',
                                    }}",
                                    app_name.replace("'", r"\'"),
                                    replaces_id,
                                    app_icon,
                                    summary.replace("'", r"\'"),
                                    body.replace("'", r"\'"),
                                    vec_to_table(actions),
                                    expire_timeout,
                                )
                            )
                        )

                    )
                    .expect("error with sending a notification event");
            })

        };
        let connection = zbus::connection::Builder::session().unwrap()
            .name("org.freedesktop.Notifications").unwrap()
            .serve_at("/org/freedesktop/Notifications", test).unwrap()
            .build()
            .await.unwrap();

        connection
            .object_server()
            .interface::<_, NotificationTest>("/org/freedesktop/Notifications")
            .await.unwrap();


        loop {
            cosmic::iced::futures::pending!();
        }
    }))
}
