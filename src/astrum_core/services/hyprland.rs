use std::{collections::HashMap, sync::{Arc, RwLock}};

use color_print::cprintln;
use cosmic::iced::{advanced::subscription, stream, Subscription};
use hyprland::{data::{Client, Clients, Workspace, Workspaces}, event_listener::AsyncEventListener, shared::{HyprData, HyprDataActive, HyprDataActiveOptional, HyprDataVec}};
use log::{debug, info};

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
            if requests.contains(&"clients".to_string()) {
                listener.add_window_opened_handler({
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
                                                StringOrNum::String("clients".to_string()),
                                                get_clients()
                                            )
                                        )
                                    )
                                    .expect("error with clients opened event")
                            }
                        })
                    }
                });

                listener.add_window_moved_handler({
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
                                                StringOrNum::String("clients".to_string()),
                                                get_clients()
                                            )
                                        )
                                    )
                                    .expect("error with clients moved event")
                            }
                        })
                    }
                });

                listener.add_window_closed_handler({
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
                                                StringOrNum::String("clients".to_string()),
                                                get_clients()
                                            )
                                        )
                                    )
                                    .expect("error with clients closed event")
                            }
                        })
                    }
                });

                listener.add_window_title_changed_handler({
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
                                                StringOrNum::String("clients".to_string()),
                                                get_clients()
                                            )
                                        )
                                    )
                                    .expect("error with clients title changed event")
                            }
                        })
                    }
                });
            }
            if requests.contains(&"active_client".to_string()) {
                listener.add_active_window_changed_handler({
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
                                                StringOrNum::String("active_client".to_string()),
                                                get_active_client()
                                            )
                                        )
                                    )
                                    .expect("error with active client event")
                            }
                        })
                    }
                });
            }
            let res = listener.start_listener_async().await;

            if let Err(e) = res {
                cprintln!("<r>restarting hyprland listener due to error: {:?}</r>", e);
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

pub fn get_active_client() -> String {
    let active_client = Client::get_active().unwrap();

    if let Some(client) = active_client {
        return format!(
            "{{
                workspace_id = {workspace_id},
                at = {{ x = {at_x}, y = {at_y} }},
                size = {{ x = {size_x}, y = {size_y} }},
                initial_title = '{window_initial_title}',
                initial_class = '{window_initial_class}',
                title = '{window_title}',
                class = '{window_class}',
                process_id = {pid},
                floating = {floating}
            }}, ",
            workspace_id = client.workspace.id,
            at_x = client.at.0,
            at_y = client.at.1,
            size_x = client.size.0,
            size_y = client.size.1,
            window_initial_title = client.initial_title.replace("'", r"\'"),
            window_initial_class = client.initial_class.replace("'", r"\'"),
            window_title = client.title.replace("'", r"\'"),
            window_class = client.class.replace("'", r"\'"),
            pid = client.pid,
            floating = client.floating
        );

    }

    "nil".to_string()
}

fn get_clients() -> String {
    let mut clients_vec = Clients::get()
        .unwrap()
        .to_vec();

    // clients.sort_by_key(|client| client.workspace.id);
    let mut clients: HashMap<i32, Vec<Client>> = HashMap::new();

    // insert a client to a vec, not to a hashmap
    // sooo maybe an if?
    // this will get ran every time something happens to a window so keep that in mind
    clients_vec.iter().for_each(|client| {
        clients.entry(client.workspace.id).or_insert(Vec::new()).push(client.clone());
    });

    let mut builder_string = "{  ".to_string();

    for (workspace_id, client_vec) in clients.iter() {
        builder_string.push_str(&format!(
            "[{id}] = {{ ",
            id = workspace_id
        ));
        for client in client_vec.iter() {
            builder_string.push_str(
                &format!(
                    "{{
                        workspace_id = {workspace_id},
                        at = {{ x = {at_x}, y = {at_y} }},
                        size = {{ x = {size_x}, y = {size_y} }},
                        initial_title = '{window_initial_title}',
                        initial_class = '{window_initial_class}',
                        title = '{window_title}',
                        class = '{window_class}',
                        process_id = {pid},
                        floating = {floating}
                    }}, ",
                    workspace_id = workspace_id,
                    at_x = client.at.0,
                    at_y = client.at.1,
                    size_x = client.size.0,
                    size_y = client.size.1,
                    window_initial_title = client.initial_title.replace("'", r"\'"),
                    window_initial_class = client.initial_class.replace("'", r"\'"),
                    window_title = client.title.replace("'", r"\'"),
                    window_class = client.class.replace("'", r"\'"),
                    pid = client.pid,
                    floating = client.floating
                )
            );
        }
        builder_string.push_str("},");
    }
    builder_string.push_str("}");

    debug!("builder string is\n {}", builder_string);
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
