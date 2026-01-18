use cosmic::widget::menu::Tree;

use crate::astrum_core::app::main::AstrumMessages;

use super::process_lua_element;



pub fn make_tree_widget<'a>(
    data: mlua::Table
) -> Tree<'a, AstrumMessages> {
    println!("processing tree");
    let item = process_lua_element(data.get::<mlua::Table>("item").expect("expected item in tree widget")).unwrap();

    let mut menu_tree_widget;
    if let Ok(children_table) = data.get::<mlua::Table>("children")  {
        let mut children = Vec::new();

        for pairs in children_table.pairs::<mlua::Number, mlua::Table>()  {
            let (_, value) = pairs.unwrap();
            if value.get::<mlua::String>("widget_name").unwrap().to_string_lossy() == "tree" {
                children.push(make_tree_widget(value));
            } else {
                let widget = process_lua_element(value);

                if let Some(element) = widget {
                    children.push(element.into());
                }
            }
        }

        menu_tree_widget = Tree::with_children(item, children);
    } else {
        menu_tree_widget = Tree::new(item);
    }

    if let Ok(width) = data.get::<mlua::Integer>("width") {
        menu_tree_widget = menu_tree_widget.width(width as u16);
    }
    if let Ok(height) = data.get::<mlua::Integer>("height") {
        menu_tree_widget = menu_tree_widget.height(height as u16);
    }

    menu_tree_widget
}
