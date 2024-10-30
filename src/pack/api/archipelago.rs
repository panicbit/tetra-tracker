use std::path::PathBuf;

use eyre::eyre;
use mlua::{ErrorContext, Function, UserData, UserDataFields, UserDataMethods, Value};
use tracing::{debug, debug_span};

pub struct Archipelago {
    root: PathBuf,
    clear_handlers: Vec<(String, Function)>,
    item_handlers: Vec<(String, Function)>,
    location_handlers: Vec<(String, Function)>,
    retrieved_handlers: Vec<(String, Function)>,
    set_reply_handlers: Vec<(String, Function)>,
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
            |_lua, this, (name, callback): (String, Value)| {
                let _span = debug_span!("Archipelago::AddClearHandler", ?name, ?callback).entered();
                let callback = callback
                    .as_function()
                    .ok_or_else(|| mlua::Error::runtime("callback must be a function"))?
                    .clone();

                this.clear_handlers.push((name, callback));

                Ok(())
            },
        );

        methods.add_method_mut(
            "AddItemHandler",
            |_lua, this, (name, callback): (String, Value)| {
                let _span = debug_span!("Archipelago::AddItemHandler", ?name, ?callback).entered();
                let callback = callback
                    .as_function()
                    .ok_or_else(|| mlua::Error::runtime("callback must be a function"))?
                    .clone();

                this.item_handlers.push((name, callback));

                Ok(())
            },
        );

        methods.add_method_mut(
            "AddLocationHandler",
            |_lua, this, (name, callback): (String, Value)| {
                let _span =
                    debug_span!("Archipelago::AddLocationHandler", ?name, ?callback).entered();
                let callback = callback
                    .as_function()
                    .ok_or_else(|| mlua::Error::runtime("callback must be a function"))?
                    .clone();

                this.location_handlers.push((name, callback));

                Ok(())
            },
        );

        methods.add_method_mut(
            "AddRetrievedHandler",
            |_lua, this, (name, callback): (String, Value)| {
                let _span =
                    debug_span!("Archipelago::AddRetrievedHandler", ?name, ?callback).entered();
                let callback = callback
                    .as_function()
                    .ok_or_else(|| mlua::Error::runtime("callback must be a function"))?
                    .clone();

                this.retrieved_handlers.push((name, callback));

                Ok(())
            },
        );

        methods.add_method_mut(
            "AddSetReplyHandler",
            |_lua, this, (name, callback): (String, Value)| {
                let _span =
                    debug_span!("Archipelago::AddSetReplyHandler", ?name, ?callback).entered();
                let callback = callback
                    .as_function()
                    .ok_or_else(|| mlua::Error::runtime("callback must be a function"))?
                    .clone();

                this.set_reply_handlers.push((name, callback));

                Ok(())
            },
        );

        methods.add_meta_method("__index", |_, _, index: Value| -> mlua::Result<()> {
            let _span = debug_span!("Archipelago::__index", ?index).entered();
            let index = index.to_string()?;

            Err(mlua::Error::runtime(format!(
                "`Archipelago.{index}` does not exist"
            )))
        });
    }
}
