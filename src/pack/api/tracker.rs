use std::path::PathBuf;
use std::{fs, iter};

use egui::ahash::HashMapExt;
use eyre::{eyre, Context};
use fnv::FnvHashMap;
use mlua::{Lua, UserData, UserDataFields, UserDataMethods};
use tracing::{debug, debug_span, error, instrument, warn};

use crate::pack::api::tracker::flat::{Location, Section};
use crate::pack::api::tracker::id::{AsId, Id};
use crate::pack::api::tracker::nested::Map;
use crate::pack::api::AccessabilityLevel;
use crate::pack::rule::{Call, Rule};
use crate::pack::VariantUID;
use crate::util::deserialize_hjson;

pub mod flat;
pub mod id;
pub mod nested;

mod stateful_item;
pub use stateful_item::StatefulItem;

pub struct Tracker {
    root: PathBuf,
    next_id: Id<()>,
    maps: Vec<Map>,
    locations: FnvHashMap<Id<Location>, Location>,
    items: Vec<StatefulItem>,
    variant_uid: VariantUID,
}

impl Tracker {
    pub fn new(root: impl Into<PathBuf>, variant_uid: &VariantUID) -> Self {
        Self {
            next_id: Id::default(),
            root: root.into(),
            maps: Vec::new(),
            locations: FnvHashMap::new(),
            items: Vec::new(),
            variant_uid: variant_uid.clone(),
        }
    }

    pub fn maps(&self) -> &[Map] {
        &self.maps
    }

    pub fn locations(&self) -> impl Iterator<Item = &Location> {
        self.locations.values()
    }

    pub fn locations_recursive(&self) -> impl Iterator<Item = &Location> {
        self.locations.values().flat_map(|location| {
            iter::once(location).chain(
                location
                    .children
                    .iter()
                    .flat_map(|child| self.location_children_recursive(child)),
            )
        })
    }

    pub fn location_children(
        &self,
        location: impl AsId<Location>,
    ) -> impl Iterator<Item = &Location> + '_ {
        let location = location.as_id();

        self.locations
            .get(&location)
            .into_iter()
            .flat_map(|location| {
                location
                    .children
                    .iter()
                    .flat_map(|child| self.locations.get(child))
            })
    }

    pub fn location_children_recursive(
        &self,
        location: impl AsId<Location>,
    ) -> Box<dyn Iterator<Item = &Location> + '_> {
        let location = location.as_id();
        Box::new(
            self.location_children(location).flat_map(|child| {
                iter::once(child).chain(self.location_children_recursive(&child.id))
            }),
        )
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

    pub fn next_id(&mut self) -> Id<()> {
        let next = self.next_id.clone();

        self.next_id.increment();

        next
    }

    pub fn add_location(
        &mut self,
        location: nested::Location,
        parent: Option<Id<Location>>,
    ) -> Id<Location> {
        let id = self.next_id().typed::<Location>();

        let location = Location {
            id: id.clone(),
            parent,
            name: location.name,
            access_rules: location.access_rules,
            map_locations: location.map_locations,
            sections: location
                .sections
                .into_iter()
                .map(|section| self.add_section(section, id.clone()))
                .collect::<Vec<_>>(),
            children: location
                .children
                .into_iter()
                .map(|location| self.add_location(location, Some(id.clone())))
                .collect::<Vec<_>>(),
        };

        self.locations.insert(id.clone(), location);

        id
    }

    pub fn add_section(&mut self, section: nested::Section, parent: Id<Location>) -> flat::Section {
        let id = self.next_id().typed::<flat::Section>();

        flat::Section {
            id,
            parent,
            name: section.name,
            access_rules: section.access_rules,
        }
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
            let items = deserialize_hjson::<Vec<nested::Item>>(&items)
                .with_context(|| eyre!("failed to parse items json at {items_path:?}"))
                .map_err(|err| mlua::Error::runtime(format!("{err:?}")))?;

            let mut items = items.into_iter().map(StatefulItem::new).collect::<Vec<_>>();

            this.items.append(&mut items);

            Ok(())
        });

        methods.add_method_mut("AddLocations", |_, this, locations_path: String| {
            let locations_path = this.root.join(locations_path);
            let locations = fs::read_to_string(&locations_path)?;
            let locations = deserialize_hjson::<Vec<nested::Location>>(&locations)
                .with_context(|| eyre!("failed to parse locations json at {locations_path:?}"))
                .map_err(|err| mlua::Error::runtime(format!("{err:?}")))?;

            for location in locations {
                let parent = None;
                this.add_location(location, parent);
            }

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
