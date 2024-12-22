use std::{env::current_exe, path::Path};
use binds::add_util_binds;
use mlua::{Lua, Table, Value};

mod binds;

pub fn get_or_create_module<'lua>
(
    lua: &'lua Lua,
    name: &str
) -> anyhow::Result<mlua::Table<'lua>> {
    let globals = lua.globals();

    let package: Table = globals.get("package")?;
    let loaded: Table = package.get("loaded")?;

    let module = loaded.get(name)?;

    match module {
        // no module was found, make a new one
        Value::Nil => {
            let module = lua.create_table()?;
            loaded.set(name, module.clone())?;
            Ok(module)
        },

        Value::Table(table) => Ok(table),

        other => anyhow::bail!(
            "cant register module {name} as package.loaded {name} is already set to a value of type {mod_type}",
            name = name,
            mod_type = other.type_name()
        ),
    }
    // println!("inner a");
}


pub fn make_lua_context(config_path: &Path) -> anyhow::Result<Lua> {
    // this loads all safe libraries in lua (everything except ffi)
    // might want to load ffi in the future, which would allow for better control for the user
    // but then there would be no safety garuantees
    let lua = Lua::new();

    let config_dir = config_path.parent().unwrap_or_else(|| Path::new("/"));
    {
        let globals = lua.globals();
        // add more packages to the lua context
        //
        // add astrum utils, in order to connect rust functions with lua functions
        let astrum_utils = get_or_create_module(&lua, "astrum_core_utils")?;

        let package: Table = globals.get("package").unwrap();
        let package_path: String = package.get("path").unwrap();
        let mut path_array: Vec<String> = package_path.split(";").map(|s| s.to_owned()).collect();

        fn prefix_path(array: &mut Vec<String>, path: &Path) {
            array.insert(0, format!("{}/?.lua", path.display()));
            array.insert(1, format!("{}/?/init.lua", path.display()));
        }

        let current_dir = current_exe().unwrap()
            .parent().unwrap()
            .parent().unwrap()
            .parent().unwrap()
            .join("src").join("lua_library");


        prefix_path(&mut path_array, &current_dir.as_path()); // add the astrum lua library to the package list
        prefix_path(&mut path_array, config_dir);

        add_util_binds(&lua, &astrum_utils);

        package
            .set("path", path_array.join(";"))?;

    }

    Ok(lua)
}
