use std::path::PathBuf;
use mlua::Lua;

pub struct ScriptManager {
    lua: mlua::Lua
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