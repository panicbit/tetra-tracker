use std::path::PathBuf;

use eyre::Context;
use mlua::{AnyUserData, ErrorContext, UserData, UserDataFields, UserDataMethods, UserDataRef};

pub struct ScriptHost {
    root: PathBuf,
}

impl ScriptHost {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }
}

impl UserData for ScriptHost {
    fn add_fields<F: UserDataFields<Self>>(_fields: &mut F) {}

    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_function(
            "LoadScript",
            |lua, (this, lua_path): (UserDataRef<Self>, String)| {
                let lua_path = this.root.join(lua_path);

                drop(this);

                lua.load(lua_path)
                    .exec()
                    .map_err(|err| err.context("LoadScript: failed to execute"))?;

                Ok(())
            },
        );

        methods.add_meta_method("__index", |_, _, index: mlua::Value| -> mlua::Result<()> {
            let index = index.to_string()?;

            Err(mlua::Error::runtime(format!(
                "`ScriptHost.{index}` does not exist"
            )))
        });
    }
}
