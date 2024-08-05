use std::{borrow::BorrowMut, cell::RefCell, collections::HashMap, env::Args};

use hyprland::{data::Workspace, event_listener::EventListener, shared::{HyprData, HyprDataActive, HyprDataVec, WorkspaceType}};
use cosmic::iced::{advanced::graphics::futures::{subscription, MaybeSend}, Subscription};

use crate::app::WindowMessages;
// TODO: Document the structs

pub struct ActiveWorkspace {
    pub id: usize,
    pub name: String
}

pub struct ActiveMonitor {
    pub id: usize,
    pub name: String
}

// maybe add initial title too??
pub struct ActiveClient {
    pub adress: String,
    pub title: String,
    pub class: String,
}


// TODO: Document this
pub struct Active {
    pub monitor: ActiveMonitor,
    pub workspace: ActiveWorkspace,
    pub client: ActiveClient,
}

// INFO: you dont need this hyprland model thing
// the entire model si defined via functions and subscriptions (aka signals)
// you use a function to get the active workspace
// or use a ufnction to get the active workspaces
// same with signals
// you use a subscription function to get a signal then map it with a app message
pub struct HyprlandModel<F>
where
        // F: Fn(Args),
        F: Fn(WorkspaceType) + 'static
    {
        pub active: Active,
        pub workspaces: Vec<Workspace>,
        // signals
        // TODO: Document
        pub workspace_changed: Option<F>,
        pub workspace_added: Option<F>,
        pub workspace_removed: Option<F>,
        pub workspace_moved: Option<F>,

        pub active_monitor_changed: Option<F>,
        pub active_window_changed: Option<F>
    }

// TODO: Document

#[derive(Debug, Clone)]
pub struct WorkspaceData {
    pub id: i32,
    pub active:  bool,
    // maybe add monitor
}

pub enum HyprlandListener {
    // WorkspacesChanged(Vec<WorkspaceData>),

    WorkspacesChanged(String), // lua string

    // ActiveWorkspaceChanged(i32),
}


pub fn get_workspaces() -> String {
    let active_workspace = get_active_workspace();
    let mut workspaces = hyprland::data::Workspaces::get()
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
    // let workspaces_data = workspaces.iter().map(|workspace| {
    //     // WorkspaceData {
    //     //     id: workspace.id,
    //     //     active: workspace.id == active_workspace,
    //     // }
    // }).collect::<Vec<WorkspaceData>>();
    // workspaces_data
    builder_string
}
pub fn get_active_workspace() -> i32 {
    let active = hyprland::data::Workspace::get_active().unwrap();

    active.id
}

pub fn change_workspace(workspace_id: i32) {
    hyprland::dispatch::Dispatch::call(
        hyprland::dispatch::DispatchType::Workspace(
            hyprland::dispatch::WorkspaceIdentifierWithSpecial::Id(workspace_id),
        )
    ).expect("Change workspace failed to dispatch call!");
}

pub fn listen_workspaces(signals_requested: HashMap<String, bool>) -> Subscription<WindowMessages>
{
    subscription::channel("workspaces-listener", 100, move |output| async move {
        let output = RefCell::new(output); // INFO: probbaly should read what a refcell is
        let mut listener = EventListener::new();

        if let Some(_bool) = signals_requested.get("workspaces") {

            listener.add_workspace_added_handler(
                {
                    let output = output.clone();
                    move |_| {
                        output
                            .borrow_mut()
                            .try_send( WindowMessages::Msg(("hyprland_workspaces".to_string(), get_workspaces() )))
                            .expect("hyprland workspaces listener failed sending signal");
                        // if let Some(response_message) = response(HyprlandListener::WorkspacesChanged(get_workspaces())) {
                        //
                        //     output
                        //         .borrow_mut()
                        //         .try_send(response_message)
                        //         .expect("write error message herel ater");
                        // }
                    }
                }

            );

            listener.add_workspace_destroy_handler({
                let output = output.clone();
                move |_| {
                    output
                        .borrow_mut()
                        .try_send( WindowMessages::Msg(("hyprland_workspaces".to_string(), get_workspaces() )))
                        .expect("hyprland workspaces listener failed sending signal");
                }
            });

            listener.add_workspace_moved_handler({
                let output = output.clone();
                move |_| {
                    output
                        .borrow_mut()
                        .try_send( WindowMessages::Msg(("hyprland_workspaces".to_string(), get_workspaces() )))
                        .expect("hyprland workspaces listener failed sending signal");
                }
            });

            listener.add_workspace_change_handler({
                let output = output.clone();
                move |_| {
                    output
                        .borrow_mut()
                        .try_send( WindowMessages::Msg(("hyprland_workspaces".to_string(), get_workspaces() )))
                        .expect("hyprland workspaces listener failed sending signal");
                }
            });
        }



        listener
            .start_listener_async()
            .await
            .expect("failed to start workspaces listener");

        panic!("Hyprland workspaces listener failed! Exiting.");
        // loop {
        //
        // }
    })
}

// make it via a channel!!!
// struct WorkspaceAdded;
// pub fn workspace_added() -> Subscription<WorkspaceType>{
//     Subscription::from_recipe(WorkspaceAdded)
// }
//
// impl subscription::Recipe for WorkspaceAdded {
//     type Output = WorkspaceType;
//
//     fn hash(&self, state: &mut iced::advanced::Hasher) {
//         use std::hash::Hash;
//
//         std::any::TypeId::of::<Self>().hash(state);
//         self.hash(state);
//     }
//
//     fn stream(
//         self: Box<Self>,
//         input: subscription::EventStream
//     ) -> iced::advanced::graphics::futures::BoxStream<Self::Output> {
//         use iced::futures::StreamExt;
//
//         let mut listener = EventListener::new();
//
//         listener.add_workspace_added_handler(|data| )
//     }
// }
