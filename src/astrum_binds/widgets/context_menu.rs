use cosmic::widget::{context_menu, ContextMenu};

use crate::astrum_core::app::main::AstrumMessages;

use super::{process_lua_element, tree::make_tree_widget};


//
// pub fn make_context_menu_widget<'a>(
//     data: mlua::Table
// ) -> ContextMenu<'a, AstrumMessages>{
//     let context = process_lua_element(data.get::<mlua::Table>("child").expect("expected child proprety in context menu")).unwrap();
//
//     let mut menu = Vec::new();
//     let children_table: mlua::Table = data.get("tree").unwrap();
//
//     for pairs in children_table.pairs::<mlua::Value, mlua::Table>()  {
//         let (_, value) = pairs.unwrap();
//         // println!("ok so tree thing should look like this");
//         // for pairs2 in value.pairs::<mlua::String, mlua::Value>() {
//         //     let (key, v) = pairs2.unwrap();
//         //
//         //     println!("got key: {:?} and value: {:?}", key, v);
//         //     if let mlua::Value::Table(v_table) = v {
//         //         for pairs3 in v_table.pairs::<mlua::String, mlua::Table>() {
//         //             let (k, v2) = pairs3.unwrap();
//         //
//         //             println!("          inside got key: {:?} and value: {:?}", k, v2);
//         //         }
//         //
//         //     }
//         //
//         // }
//         let widget = make_tree_widget(value);
//
//         menu.push(widget);
//     }
//
//     context_menu(context, Some(menu))
// }
