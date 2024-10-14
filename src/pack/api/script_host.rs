use std::path::PathBuf;

use mlua::{UserData, UserDataFields, UserDataMethods};

pub struct ScriptHost {
    root: PathBuf,
}

impl ScriptHost {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }
}

impl UserData for ScriptHost {
    fn add_fields<'lua, F: UserDataFields<'lua, Self>>(_fields: &mut F) {}

    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("LoadScript", |lua, this, lua_path: String| {
            let lua_path = this.root.join(lua_path);

            lua.load(lua_path).exec()?;

            Ok(())
        });

        methods.add_meta_method("__index", |_, _, index: mlua::Value| -> mlua::Result<()> {
            let index = index.to_string()?;

            Err(mlua::Error::runtime(format!(
                "`ScriptHost.{index}` does not exist"
            )))
        });
    }
}
