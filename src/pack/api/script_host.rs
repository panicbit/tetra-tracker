use std::path::PathBuf;

use mlua::{ErrorContext, UserData, UserDataFields, UserDataMethods, UserDataRef};
use tracing::{debug, debug_span, info_span, trace};

pub struct ScriptHost {
    root: PathBuf,
}

impl ScriptHost {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }
}

impl Drop for ScriptHost {
    fn drop(&mut self) {
        debug!("Dropping ScriptHost");
    }
}

impl UserData for ScriptHost {
    fn add_fields<F: UserDataFields<Self>>(_fields: &mut F) {}

    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_function(
            "LoadScript",
            |lua, (this, path): (UserDataRef<Self>, String)| {
                let _span = debug_span!("ScriptHost::LoadScript", path = path).entered();
                let path = this.root.join(path);

                drop(this);

                info_span!("lua").in_scope(|| {
                    trace!("Start executing");

                    let result = lua
                        .load(path)
                        .exec()
                        .map_err(|err| err.context("LoadScript: failed to execute"));

                    trace!("End executing");

                    result
                })?;

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
