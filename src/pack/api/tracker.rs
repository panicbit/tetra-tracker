use std::path::PathBuf;
use std::{fs, iter};

use eyre::{eyre, Context};
use mlua::{Lua, UserData, UserDataFields, UserDataMethods};
use tracing::{debug, debug_span, error, instrument, warn};

use crate::pack::rule::{Call, Rule};
use crate::pack::VariantUID;
use crate::util::deserialize_hjson;

mod item;
pub use item::Item;

mod map;
pub use map::{LocationShape, Map};

mod location;
pub use location::Location;

mod map_location;
pub use map_location::MapLocation;

mod section;
pub use section::Section;

mod stateful_item;
pub use stateful_item::StatefulItem;

pub struct Tracker {
    root: PathBuf,
    maps: Vec<Map>,
    locations: Vec<Location>,
    items: Vec<StatefulItem>,
    variant_uid: VariantUID,
}

impl Tracker {
    pub fn new(root: impl Into<PathBuf>, variant_uid: &VariantUID) -> Self {
        Self {
            root: root.into(),
            maps: Vec::new(),
            locations: Vec::new(),
            items: Vec::new(),
            variant_uid: variant_uid.clone(),
        }
    }

    pub fn maps(&self) -> &[Map] {
        &self.maps
    }

    pub fn locations(&self) -> &[Location] {
        &self.locations
    }

    pub fn locations_recursive(&self) -> impl Iterator<Item = &Location> {
        self.locations
            .iter()
            .flat_map(|location| iter::once(location).chain(location.child_locations_recursive()))
    }

    #[instrument(level = "error", skip(self))]
    pub fn provider_count_for_code(&self, lua: &Lua, code: &str) -> i32 {
        let rule = match code.parse::<Rule>() {
            Ok(rule) => rule,
            Err(err) => {
                error!("invalid code: {err:?}");
                return 0;
            }
        };

        match rule {
            Rule::Call(call) => self.provider_count_for_call(lua, &call),
            Rule::Item(name) => self.provider_count_for_item(&name),
            _ => {
                error!("invalid code (only lua calls and item code allowed)");
                0
            }
        }
    }

    #[instrument(level = "error", skip(self))]
    pub fn provider_count_for_call(&self, lua: &Lua, call: &Call) -> i32 {
        match call.exec::<i32>(lua) {
            Ok(count) => count,
            Err(err) => {
                error!("failed to call `{}`: {err:?}", call.name);
                0
            }
        }
    }

    #[instrument(level = "error", skip(self))]
    pub fn provider_count_for_item(&self, item_code: &str) -> i32 {
        let mut count = 0;

        for item in &self.items {
            count += item.provider_count(item_code);
        }

        count
    }
}

impl Drop for Tracker {
    fn drop(&mut self) {
        debug!("Dropping Tracker");
    }
}

impl UserData for Tracker {
    fn add_fields<F: UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("ActiveVariantUID", |_, this| {
            Ok(this.variant_uid.as_str().to_owned())
        });
    }

    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method_mut("AddMaps", |_, this, maps_path: String| {
            let _span = debug_span!("Tracker::AddMaps").entered();
            let maps_path = this.root.join(maps_path);
            let maps = fs::read_to_string(&maps_path)?;
            let mut maps = deserialize_hjson::<Vec<Map>>(&maps)
                .with_context(|| eyre!("failed to parse maps json at {maps_path:?}"))
                .map_err(|err| mlua::Error::runtime(format!("{err:?}")))?;

            this.maps.append(&mut maps);

            Ok(())
        });

        methods.add_method_mut("AddItems", |_, this, items_path: String| {
            let items_path = this.root.join(items_path);
            let items = fs::read_to_string(&items_path)?;
            let items = deserialize_hjson::<Vec<Item>>(&items)
                .with_context(|| eyre!("failed to parse items json at {items_path:?}"))
                .map_err(|err| mlua::Error::runtime(format!("{err:?}")))?;

            let mut items = items.into_iter().map(StatefulItem::new).collect::<Vec<_>>();

            this.items.append(&mut items);

            Ok(())
        });

        methods.add_method_mut("AddLocations", |_, this, locations_path: String| {
            let locations_path = this.root.join(locations_path);
            let locations = fs::read_to_string(&locations_path)?;
            let mut locations = deserialize_hjson::<Vec<Location>>(&locations)
                .with_context(|| eyre!("failed to parse locations json at {locations_path:?}"))
                .map_err(|err| mlua::Error::runtime(format!("{err:?}")))?;

            this.locations.append(&mut locations);

            Ok(())
        });

        methods.add_method_mut("AddLayouts", |_, _tehis, _layouts_path: String| {
            let _span = debug_span!("Tracker::AddLayouts").entered();
            warn!("TODO: implement");

            Ok(())
        });

        methods.add_meta_method("__index", |_, _, index: mlua::Value| -> mlua::Result<()> {
            let index = index.to_string()?;

            Err(mlua::Error::runtime(format!(
                "`Tracker.{index}` does not exist"
            )))
        });
    }
}
