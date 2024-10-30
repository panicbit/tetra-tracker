use std::path::PathBuf;

use mlua::{Lua, RegistryKey, UserData, UserDataFields, UserDataMethods};
use tracing::{debug, error};

pub struct Archipelago {
    root: PathBuf,
    clear_handlers: Vec<(String, RegistryKey)>,
    item_handlers: Vec<(String, RegistryKey)>,
    location_handlers: Vec<(String, RegistryKey)>,
    retrieved_handlers: Vec<(String, RegistryKey)>,
    set_reply_handlers: Vec<(String, RegistryKey)>,
}

impl Archipelago {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self {
            root: root.into(),
            clear_handlers: Vec::new(),
            item_handlers: Vec::new(),
            location_handlers: Vec::new(),
            retrieved_handlers: Vec::new(),
            set_reply_handlers: Vec::new(),
        }
    }

    pub fn clear(&mut self, lua: &Lua) {
        let handler_sets = [
            &mut self.clear_handlers,
            &mut self.item_handlers,
            &mut self.location_handlers,
            &mut self.retrieved_handlers,
            &mut self.set_reply_handlers,
        ];

        for handler_set in handler_sets {
            for (name, handler) in handler_set.drain(..) {
                if let Err(err) = lua.remove_registry_value(handler) {
                    error!("Failed to remove handler `{name}` from the registry {err:?}")
                }
            }
        }
    }
}

impl Drop for Archipelago {
    fn drop(&mut self) {
        debug!("Dropping Archipelago userdata");
    }
}

impl UserData for Archipelago {
    fn add_fields<F: UserDataFields<Self>>(_fields: &mut F) {}

    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method_mut(
            "AddClearHandler",
            |lua, this, (name, callback): (String, mlua::Value)| {
                let callback = callback
                    .as_function()
                    .ok_or_else(|| mlua::Error::runtime("callback must be a function"))?;

                let callback = lua.create_registry_value(callback)?;

                this.clear_handlers.push((name, callback));

                Ok(())
            },
        );

        methods.add_method_mut(
            "AddItemHandler",
            |lua, this, (name, callback): (String, mlua::Value)| {
                let callback = callback
                    .as_function()
                    .ok_or_else(|| mlua::Error::runtime("callback must be a function"))?;

                let callback = lua.create_registry_value(callback)?;

                this.item_handlers.push((name, callback));

                Ok(())
            },
        );

        methods.add_method_mut(
            "AddLocationHandler",
            |lua, this, (name, callback): (String, mlua::Value)| {
                let callback = callback
                    .as_function()
                    .ok_or_else(|| mlua::Error::runtime("callback must be a function"))?;

                let callback = lua.create_registry_value(callback)?;

                this.location_handlers.push((name, callback));

                Ok(())
            },
        );

        methods.add_method_mut(
            "AddRetrievedHandler",
            |lua, this, (name, callback): (String, mlua::Value)| {
                let callback = callback
                    .as_function()
                    .ok_or_else(|| mlua::Error::runtime("callback must be a function"))?;

                let callback = lua.create_registry_value(callback)?;

                this.retrieved_handlers.push((name, callback));

                Ok(())
            },
        );

        methods.add_method_mut(
            "AddSetReplyHandler",
            |lua, this, (name, callback): (String, mlua::Value)| {
                let callback = callback
                    .as_function()
                    .ok_or_else(|| mlua::Error::runtime("callback must be a function"))?;

                let callback = lua.create_registry_value(callback)?;

                this.set_reply_handlers.push((name, callback));

                Ok(())
            },
        );

        methods.add_meta_method("__index", |_, _, index: mlua::Value| -> mlua::Result<()> {
            let index = index.to_string()?;

            Err(mlua::Error::runtime(format!(
                "`Archipelago.{index}` does not exist"
            )))
        });
    }
}
