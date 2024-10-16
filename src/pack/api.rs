use std::path::PathBuf;

use eyre::{Context, Result};
use mlua::{AnyUserData, Lua, MultiValue};

use archipelago::Archipelago;
use script_host::ScriptHost;
use tracing::{info, warn};
pub use tracker::Tracker;

use crate::pack::VariantUID;

mod archipelago;
mod script_host;
pub mod tracker;

pub struct Api {
    lua: Lua,
}

impl Api {
    pub fn new(root: impl Into<PathBuf>, variant_uid: &VariantUID) -> Result<Self> {
        let root = root.into();
        let lua = Lua::new();
        let globals = lua.globals();

        lua.set_warning_function(|_lua, msg, _to_continue| {
            warn!("{msg}");
            Ok(())
        });

        let print = lua.create_function(|_lua, values: MultiValue| {
            let messages = values
                .into_iter()
                .map(|value| value.to_string())
                .collect::<Result<Vec<_>, _>>()?;
            let message = messages.as_slice().join("\t");

            info!("{message}");

            Ok(())
        })?;

        globals.set("print", print)?;

        globals
            .set("ScriptHost", ScriptHost::new(&root))
            .context("failed to set ScriptHost global")?;
        globals
            .set("Archipelago", Archipelago::new(&root))
            .context("failed to set Archipelago global")?;
        globals
            .set("Tracker", Tracker::new(root, variant_uid))
            .context("failed to set Archipelago global")?;

        drop(globals);

        Ok(Self { lua })
    }

    pub fn with_tracker<F, R>(&self, f: F) -> Result<R>
    where
        F: FnOnce(&Tracker) -> R,
    {
        let tracker = self
            .lua
            .globals()
            .get::<_, AnyUserData>("Tracker")
            .context("failed to get `Tracker` global")?;
        let tracker = tracker
            .borrow::<Tracker>()
            .context("failed to borrow tracker mutably")?;

        let result = f(&tracker);

        Ok(result)
    }

    pub fn with_tracker_mut<F, R>(&self, f: F) -> Result<R>
    where
        F: FnOnce(&mut Tracker) -> R,
    {
        let tracker = self
            .lua
            .globals()
            .get::<_, AnyUserData>("Tracker")
            .context("failed to get `Tracker` global")?;
        let mut tracker = tracker
            .borrow_mut::<Tracker>()
            .context("failed to borrow tracker mutably")?;

        let result = f(&mut tracker);

        Ok(result)
    }

    pub fn lua(&self) -> &Lua {
        &self.lua
    }
}
