use std::sync::{Arc, RwLock};

use color_print::cprintln;
use cosmic::{app::Message, iced::{advanced::subscription, stream, Subscription}};
use hyprland::{data::{Workspace, Workspaces}, event_listener::AsyncEventListener, shared::{HyprData, HyprDataActive, HyprDataVec}};

use crate::astrum_core::app::main::{AstrumMessages, StringOrNum};

// TODO: once hyprland-rs 0.4 releases
// rewrite the whole service,


pub fn hyprland_service_channel(requests: Vec<String>) -> Subscription<AstrumMessages> {
    Subscription::run_with_id(1, stream::channel(100, |mut output| async move {
        let output = Arc::new(RwLock::new(output));
        loop {
            let mut listener = AsyncEventListener::new();

            if requests.contains(&"workspaces".to_string()) {
                listener.add_workspace_added_handler({
                    let output = output.clone();

                    move |_| {
                        let output = output.clone();
                        Box::pin(async move {
                            if let Ok(mut output) = output.write() {
                                output
                                    .try_send(
                                        AstrumMessages::SubscriptionRequest(
                                            (
                                                "hyprland".to_string(),
                                                StringOrNum::String("workspaces".to_string()),
                                                get_workspaces()
                                            )
                                        )
                                    )
                                    .expect("error with workspace added event")
                            }
                        })
                    }
                });

                listener.add_workspace_changed_handler({
                    let output = output.clone();

                    move |_| {
                        let output = output.clone();
                        Box::pin(async move {
                            if let Ok(mut output) = output.write() {
                                output
                                    .try_send(
                                        AstrumMessages::SubscriptionRequest(
                                            (
                                                "hyprland".to_string(),
                                                StringOrNum::String("workspaces".to_string()),
                                                get_workspaces()
                                            )
                                        )
                                    )
                                    .expect("error with workspace changed event")
                            }
                        })
                    }
                });

                listener.add_workspace_deleted_handler({
                    let output = output.clone();

                    move |_| {
                        let output = output.clone();
                        Box::pin(async move {
                            if let Ok(mut output) = output.write() {
                                output
                                    .try_send(
                                        AstrumMessages::SubscriptionRequest(
                                            (
                                                "hyprland".to_string(),
                                                StringOrNum::String("workspaces".to_string()),
                                                get_workspaces()
                                            )
                                        )
                                    )
                                    .expect("error with workspace deleted event")
                            }
                        })
                    }
                });

                listener.add_workspace_moved_handler({
                    let output = output.clone();

                    move |_| {
                        let output = output.clone();
                        Box::pin(async move {
                            if let Ok(mut output) = output.write() {
                                output
                                    .try_send(
                                        AstrumMessages::SubscriptionRequest(
                                            (
                                                "hyprland".to_string(),
                                                StringOrNum::String("workspaces".to_string()),
                                                get_workspaces()
                                            )
                                        )
                                    )
                                    .expect("error with workspace moved event")
                            }
                        })
                    }
                });

            }
            let res = listener.start_listener_async().await;

            if let Err(e) = res {
                cprintln!("<r>restarting workspaces listener due to error: {:?}</r>", e);
            }

        }
    }))
}

pub fn get_active_workspace() -> i32 {
    let active = Workspace::get_active().unwrap();

    active.id
}

fn get_workspaces() -> String {
    let active_workspace = get_active_workspace();
    let mut workspaces = Workspaces::get()
        .unwrap()
        .to_vec();
    workspaces.sort_by_key(|w| w.id);
    let mut builder_string = "{ ".to_string();
    for workspace in workspaces.iter() {
        builder_string.push_str(
            &(format!(
                "{{ id = {id}, active = {active} }}, ",
                id = workspace.id,
                active = workspace.id == active_workspace,
            ))

        );
    }
    builder_string.push_str("}");

    builder_string

}

pub fn change_workspace(workspace_id: i32) {
    hyprland::dispatch::Dispatch::call(
        hyprland::dispatch::DispatchType::Workspace(
            hyprland::dispatch::WorkspaceIdentifierWithSpecial::Id(workspace_id),
        )
    ).expect("Change workspace failed to dispatch call!");
}

// pub fn hyprland_service_channel(requests: Vec<String>) -> Subscription<Message<AstrumMessages>> {
//
//     stream::channel(100, |mut output| async move {
//         loop {
//             // TODO: once hyprland-rs is 0.4 (or once i bother to switch to it)
//             // start working on bindings for this
//         }
//     })
// }
