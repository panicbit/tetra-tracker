use std::fmt::Debug;
use std::path::PathBuf;

use eyre::{Context, Result};
use mlua::{AnyUserData, FromLua, IntoLua, Lua, LuaOptions, MultiValue, StdLib, Table, Value};

use archipelago::Archipelago;
use script_host::ScriptHost;
use strum::{EnumIs, FromRepr};
use tracing::{info, instrument};
pub use tracker::Tracker;

use crate::pack::VariantUID;

mod archipelago;
mod script_host;
pub mod tracker;

pub struct Api {
    lua: Lua,
}

impl Api {
    #[instrument]
    pub fn new(root: impl Into<PathBuf> + Debug, variant_uid: &VariantUID) -> Result<Self> {
        let root = root.into();
        let options = LuaOptions::default();

        let lua = Lua::new_with(stdlib(), options).context("failed to create lua state")?;
        let globals = lua.globals();

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

        globals.set("AccessibilityLevel", AccessabilityLevel::table(&lua)?)?;

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

        lua.sandbox(true).context("failed to enable sandbox mode")?;

        Ok(Self { lua })
    }

    #[instrument(skip_all)]
    pub fn with_tracker<F, R>(&self, f: F) -> Result<R>
    where
        F: FnOnce(&Tracker) -> R,
    {
        let tracker = self
            .lua
            .globals()
            .get::<AnyUserData>("Tracker")
            .context("failed to get `Tracker` global")?;
        let tracker = tracker
            .borrow::<Tracker>()
            .context("failed to borrow tracker immutably")?;

        let result = f(&tracker);

        Ok(result)
    }

    #[instrument(skip_all)]
    pub fn with_tracker_mut<F, R>(&self, f: F) -> Result<R>
    where
        F: FnOnce(&mut Tracker) -> R,
    {
        let tracker = self
            .lua
            .globals()
            .get::<AnyUserData>("Tracker")
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

fn stdlib() -> StdLib {
    [
        StdLib::STRING,
        StdLib::TABLE,
        StdLib::UTF8,
        StdLib::MATH,
        // StdLib::IO,
        StdLib::PACKAGE,
    ]
    .into_iter()
    .fold(StdLib::NONE, |libs, lib| libs | lib)
}

#[derive(FromRepr, EnumIs, Copy, Clone)]
#[repr(i32)]
pub enum AccessabilityLevel {
    None = 0,
    Partial = 1,
    Inspect = 2,
    SequenceBreak = 3,
    Normal = 4,
    Cleared = 5,
}

impl AccessabilityLevel {
    pub fn table(lua: &Lua) -> mlua::Result<Table> {
        lua.create_table_from([
            ("None", Self::None),
            ("Partial", Self::Partial),
            ("Inspect", Self::Inspect),
            ("SequenceBreak", Self::SequenceBreak),
            ("Normal", Self::Normal),
            ("Cleared", Self::Cleared),
        ])
    }
}

impl IntoLua for AccessabilityLevel {
    fn into_lua(self, _lua: &Lua) -> mlua::Result<mlua::Value> {
        Ok(Value::Integer(self as i32))
    }
}

impl IntoLua for &'_ AccessabilityLevel {
    fn into_lua(self, _lua: &Lua) -> mlua::Result<mlua::Value> {
        Ok(Value::Integer(*self as i32))
    }
}

impl FromLua for AccessabilityLevel {
    fn from_lua(value: Value, _lua: &Lua) -> mlua::Result<Self> {
        value
            .as_i32()
            .and_then(AccessabilityLevel::from_repr)
            .ok_or_else(|| mlua::Error::runtime(format!("invalid accessibility level: {value:?}")))
    }
}
