use std::any::type_name;
use std::path::PathBuf;

use mlua::{Error, FromLua, Lua, Table};

pub const ROOT_TABLE_NAME: &'static str = "uth";

pub struct ScriptManager {
    lua: mlua::Lua,
    app_script_root: PathBuf,
}

impl ScriptManager {
    pub fn new(app_script_root: PathBuf) -> Self {
        let lua = Lua::new();

        Self {
            lua,
            app_script_root,
        }
    }
}

pub fn get_from_tables<'lua, T: FromLua<'lua>>(table: &'lua Table, keys: &[&str]) -> mlua::Result<T> {
    if let Some((last, prefix)) = keys.split_last() {
        let mut cur = table;
        let mut result;
        for x in prefix {
            result = cur.get::<_, Table>(*x);
            match result {
                Ok(ref t) => cur = t,
                Err(e) => {
                    log::debug!("Try get table with key {} but get error", x);
                    return Err(e);
                }
            }
        }
        cur.get(*last)
    } else {
        Err(Error::FromLuaConversionError {
            from: "Empty key (nil)",
            to: type_name::<T>(),
            message: None,
        })
    }
}