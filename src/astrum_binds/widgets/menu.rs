use crate::astrum_core::app::main::AstrumMessages;

use super::{process_lua_element, tree::make_tree_widget};



// pub fn make_menu_widget<'a>(
//     data: mlua::Table
// ) -> cosmic::widget::menu::MenuBar<'a, AstrumMessages>{
//     let children_table: mlua::Table = data.get("children").unwrap();
//     let mut widgets = Vec::new();
//
//     for pairs in children_table.pairs::<mlua::Integer, mlua::Table>()  {
//         let (_, value) = pairs.unwrap();
//         let widget = make_tree_widget(value);
//
//         widgets.push(widget);
//     }
//
//     cosmic::widget::menu::bar(widgets)
// }
