use std::any::type_name;
use std::borrow::Borrow;
use std::path::PathBuf;

use mlua::{Error, FromLua, Lua, Table};

use crate::{GlobalState, LoopState};
use crate::resource::Progress;

pub const ROOT_TABLE_NAME: &'static str = "uth";

pub struct ServerScriptManager {
    lua: mlua::Lua,
    app_script_root: PathBuf,
}

impl ServerScriptManager {
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

pub fn init_client_lua(lua: &Lua) {
    // setup global user data
    let lua_g = lua.globals();
    let init_source = format!("{}={}", ROOT_TABLE_NAME, r#"{menu={loopState={}}}"#);
    let init_chunk = lua.load(&init_source);
    init_chunk.exec().unwrap();
    let root_table = lua_g.get::<_, Table>(ROOT_TABLE_NAME).expect("Get root table failed");


    root_table.set("getFont", lua.create_function(|lua, id: String| {
        if let Some(global) = lua.app_data_ref::<GlobalState>() {
            Ok(global.handles.get_font_to_lua(&id))
        } else {
            Err(mlua::Error::UserDataDestructed)
        }
    }).unwrap()).unwrap();
    root_table.set("createProgress", lua.create_function(|lua, ()| {
        if let Some(global) = lua.app_data_ref::<GlobalState>() {
            Ok(crate::resource::CounterProgress::default())
        } else {
            Err(mlua::Error::UserDataDestructed)
        }
    }).unwrap()).unwrap();
    root_table.set("loadTexture", lua.create_function(
        |lua, (id, path, progress): (String, String, Option<crate::resource::CounterProgress>)| {
            if let Some(global) = lua.app_data_ref::<GlobalState>() {
                if let Some(progress) = progress {
                    global.handles.clone().load_texture(id, path, global.borrow(), progress.create_tracker());
                } else {
                    global.handles.clone().load_texture(id, path, global.borrow(), ());
                }
                Ok(())
            } else {
                Err(mlua::Error::UserDataDestructed)
            }
        }).unwrap()).unwrap();
    match get_from_tables::<Table>(&root_table, &["menu", "loopState"]) {
        Ok(loop_state_table) => {
            loop_state_table.set("wait", LoopState::WAIT).unwrap();
            loop_state_table.set("waitAll", LoopState::WAIT_ALL).unwrap();
            loop_state_table.set("poll", LoopState::POLL).unwrap();
            loop_state_table.set("pollNoRender", LoopState::POLL_WITHOUT_RENDER).unwrap();
        }
        Err(e) => {
            log::error!("Get table failed for {:?}", e);
        }
    };
}