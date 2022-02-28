use std::any::type_name;
use std::borrow::Borrow;
use std::path::PathBuf;

use mlua::{Error, FromLua, Lua, Table, UserData};
use wgpu_glyph::{HorizontalAlign, VerticalAlign};

use crate::{GlobalState, LoopState};
use crate::resource::{FontWrapper, Progress};

pub const ROOT_TABLE_NAME: &'static str = "uth";

#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
struct HorizontalAlignWrapper(HorizontalAlign);

#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
struct VerticalAlignWrapper(VerticalAlign);

impl From<&HorizontalAlign> for HorizontalAlignWrapper {
    fn from(v: &HorizontalAlign) -> Self {
        Self { 0: *v }
    }
}

impl UserData for HorizontalAlignWrapper {}

impl UserData for VerticalAlignWrapper {}

impl From<&VerticalAlign> for VerticalAlignWrapper {
    fn from(v: &VerticalAlign) -> Self {
        Self { 0: *v }
    }
}

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
    let init_source = format!("{}={}", ROOT_TABLE_NAME, r#"{menu={loopState={}}, textAlign={}}"#);
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
        Ok(crate::resource::CounterProgress::default())
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
    root_table.set("drawText", lua.create_function(
        |lua, (text, font, x, y, bx, by, ha, va): (String, FontWrapper, f32, f32, f32, f32, HorizontalAlignWrapper, VerticalAlignWrapper)| {
            if let Some(global) = lua.app_data_ref::<GlobalState>() {
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

    match get_from_tables::<Table>(&root_table, &["textAlign"]) {
        Ok(align_table) => {
            align_table.set("left", HorizontalAlignWrapper(HorizontalAlign::Left)).unwrap();
            align_table.set("right", HorizontalAlignWrapper(HorizontalAlign::Right)).unwrap();
            align_table.set("hCenter", HorizontalAlignWrapper(HorizontalAlign::Center)).unwrap();
            align_table.set("top", VerticalAlignWrapper(VerticalAlign::Top)).unwrap();
            align_table.set("bottom", VerticalAlignWrapper(VerticalAlign::Bottom)).unwrap();
            align_table.set("vCenter", VerticalAlignWrapper(VerticalAlign::Center)).unwrap();
        }
        Err(e) => {
            log::error!("Get table failed for {:?}", e);
        }
    };
}