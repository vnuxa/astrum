use std::collections::HashMap;
use cosmic::iced::Subscription;
use cosmic::iced::subscription;
use niri_ipc::socket::Socket;
use niri_ipc::Event;
use niri_ipc::Response;
use niri_ipc::Window;
use niri_ipc::Workspace;

use crate::app::WindowMessages;

use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};

lazy_static!{
    static ref ACTIVE_WORKSPACE: Arc<Mutex<u64>> = Arc::new(Mutex::new(1));
}


fn workspaces_to_table(mut workspaces: Vec<Workspace>) -> String {
    workspaces.sort_by_key(|w| w.idx);
    let mut builder_string = "{ ".to_string();

    for workspace in workspaces.iter() {
        let mut table = format!(
            "{{
                id = {idx},
                unique_id = {uid},
                active = {active},
                focused = {focus},
            ",
            idx = workspace.idx,
            uid = workspace.id,
            active = workspace.is_active,
            focus = workspace.is_focused
        );
        if workspace.name.is_some() {
            table.push_str(&format!("name = '{}',", workspace.name.as_ref().unwrap()));
        }
        if workspace.active_window_id.is_some() {
            table.push_str(&format!("active_window_id = {},", workspace.active_window_id.unwrap()));
        }


        table.push_str("},");

        builder_string.push_str(&table);
    }
    builder_string.push_str("}");

    builder_string
}

fn windows_to_table(windows: Vec<Window>) -> String {
    let mut builder_string = "{ ".to_string();

    for window in windows.iter() {
        let mut table = format!(
            "{{
                id = {id},
                is_focused = {focus},
            ",
            id = window.id,
            focus = window.is_focused
        );
        if window.title.is_some() {
            table.push_str(&format!("title = '{}',", window.title.as_ref().unwrap()));
        }
        if window.app_id.is_some() {
            table.push_str(&format!("app_id = '{}',", window.app_id.as_ref().unwrap()));
        }
        if window.workspace_id.is_some() {
            table.push_str(&format!("workspace_id = {},", window.workspace_id.unwrap()));
        }


        table.push_str("},");

        builder_string.push_str(&table);
    }
    builder_string.push_str("}");

    builder_string
}

pub fn niri_get_workspaces() -> String {
    let socket = Socket::connect().expect("failed to connect with niri-ipc socket");

    let (niri_result, _niri_func) = socket.send(niri_ipc::Request::Workspaces).expect("failed to send workspaces request to niri");

    if let Ok(result) = niri_result {
        if let Response::Workspaces(workspaces) = result {
            return workspaces_to_table(workspaces);
        }
    }
    panic!("getting workspaces failed?")
}

pub fn niri_get_active() -> u64 {
    let active = ACTIVE_WORKSPACE.lock().unwrap();

    return active.clone();
}
// INFO: right now this doesnt contain all of the niri-ipc events
// because im only including which are nesscessary for now
// if you want me to add more, open a PR and ill gladly add it

pub fn niri_switch_to_workspace(workspace_id: u64) {
    let socket = Socket::connect().expect("failed to connect with niri-ipc socket");

    let _result = socket.send(
        niri_ipc::Request::Action(
            niri_ipc::Action::FocusWorkspace { reference: niri_ipc::WorkspaceReferenceArg::Id(workspace_id) }
        )
    );
}
pub fn niri_focus_window(window_id: u64) {
    let socket = Socket::connect().expect("failed to connect with niri-ipc socket");

    let _result = socket.send(
        niri_ipc::Request::Action(
            niri_ipc::Action::FocusWindow { id: window_id }
        )
    );
}
pub fn niri_focus_workspace_up() {
    let socket = Socket::connect().expect("failed to connect with niri-ipc socket");

    let _result = socket.send(
        niri_ipc::Request::Action(
            niri_ipc::Action::FocusWorkspaceUp {  }
        )
    );
}
pub fn niri_focus_workspace_down() {
    let socket = Socket::connect().expect("failed to connect with niri-ipc socket");

    let _result = socket.send(
        niri_ipc::Request::Action(
            niri_ipc::Action::FocusWorkspaceDown {  }
        )
    );
}

pub fn niri_get_windows() -> String {
    let socket = Socket::connect().expect("failed to connect with niri-ipc socket");

    let (niri_result, _niri_func) = socket.send(niri_ipc::Request::Windows).expect("failed to send windows request to niri");

    if let Ok(result) = niri_result {
        if let Response::Windows(windows) = result {
            return windows_to_table(windows);
        }
    }
    panic!("getting windows failed?")
}

pub fn listen_to_niri(requested_signals: HashMap<String, bool>) -> Subscription<WindowMessages> {
    subscription::channel("astrum-niri-listener", 100, move |mut output| async move {

        tokio::task::spawn_blocking(move || {
            let niri_socket = Socket::connect().expect("failed to connect with niri-ipc socket");

            let (niri_result, mut niri_func) = niri_socket.send(niri_ipc::Request::EventStream).expect("failed to send to niri socket");

            loop {
                let event = niri_func().expect("failed to get an event back within niri-ipc");

                match event {
                    Event::WorkspacesChanged { workspaces } => {
                        println!("workspaces changed!\n heres niri: {:?}", workspaces);
                        if requested_signals.get("workspaces_changed").is_some() {
                            output
                                .try_send(
                                    WindowMessages::Msg((
                                        "niri_workspaces_changed".to_string(),
                                        workspaces_to_table(workspaces)
                                    ))
                                );
                        }
                    },
                    Event::WorkspaceActivated { id, focused } => {
                        if requested_signals.get("workspaces_changed").is_some() {
                            let mut active = ACTIVE_WORKSPACE.lock().unwrap();
                            println!("active is {:?}", active);
                            *active = id;
                            println!("changed active workspace! to {}", id);
                            println!("--------");

                            output
                                .try_send(
                                    WindowMessages::Msg((
                                        "niri_workspaces_changed".to_string(),
                                        niri_get_workspaces()
                                    ))
                                );
                        }
                    }
                    Event::WindowsChanged { windows } => {
                        println!("windows changed!");
                        if requested_signals.get("windows_changed").is_some() {
                            output
                                .try_send(
                                    WindowMessages::Msg((
                                        "niri_windows_changed".to_string(),
                                        windows_to_table(windows)
                                    ))
                                );
                        }
                    },
                    Event::WorkspaceActiveWindowChanged { workspace_id, active_window_id } => {
                        if requested_signals.get("windows_changed").is_some() {
                            output
                                .try_send(
                                    WindowMessages::Msg((
                                        "niri_windows_changed".to_string(),
                                        niri_get_windows()
                                    ))
                                );
                        }
                    }
                    _ => {}
                    // dont know if nesscessary
                    // Event::WorkspaceActiveWindowChanged { workspace_id, active_window_id } => {
                    //     if requested_signals.get("active_window_changed") {
                    //         output
                    //             .try_send(
                    //                 WindowMessages::Msg((
                    //                     "niri_active_window_changed".to_string()
                    //
                    //                 ))
                    //             )
                    //     }
                    // }
                }
            }
        }).await.unwrap();

        loop {
            cosmic::iced::futures::pending!();
        }
    })
}
