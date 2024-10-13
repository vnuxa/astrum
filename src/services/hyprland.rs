use std::{borrow::BorrowMut, cell::RefCell, collections::HashMap, env::Args, sync::{Arc, RwLock}};

use color_print::cprintln;
use hyprland::{data::Workspace, event_listener::{AsyncEventListener, EventListener}, shared::{HyprData, HyprDataActive, HyprDataVec, WorkspaceType}};
use cosmic::iced::{advanced::graphics::futures::{subscription, MaybeSend}, Subscription};

use crate::app::WindowMessages;

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
        let output = Arc::new(RwLock::new(output));
        loop {
            let mut listener = AsyncEventListener::new();
            if let Some(_bool) = signals_requested.get("workspaces") {

                listener.add_workspace_added_handler({
                    let output = output.clone();
                    move |_| {
                        let output = output.clone();
                        Box::pin(async move {
                            if let Ok(mut output) = output.write() {
                                output
                                    .try_send( WindowMessages::Msg(("hyprland_workspaces".to_string(), get_workspaces() )))
                                    .expect("error getting workspaces: workspace added event");
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
                                    .try_send( WindowMessages::Msg(("hyprland_workspaces".to_string(), get_workspaces() )))
                                    .expect("error getting workspaces: workspace change event");
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
                                    .try_send( WindowMessages::Msg(("hyprland_workspaces".to_string(), get_workspaces() )))
                                    .expect("error getting workspaces: workspace destroy event");
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
                                    .try_send( WindowMessages::Msg(("hyprland_workspaces".to_string(), get_workspaces() )))
                                    .expect("error getting workspaces: workspace moved event");
                            }
                        })
                    }
                });

                let res = listener.start_listener_async().await;

                if let Err(e) = res {
                    cprintln!("<r>restarting workspaces listener due to error: {:?}</r>", e);
                }

            }

        }

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
