use core::fmt;
use std::path::PathBuf;
use std::str::FromStr;
use std::{default, fs};

use eyre::{eyre, Context};
use mlua::{UserData, UserDataFields, UserDataMethods};
use serde::{de, Deserialize, Deserializer, Serialize};

use crate::pack::VariantUID;
use crate::util::value_or_string;
use crate::BOM;

mod map;
pub use map::{LocationShape, Map};

mod location;
pub use location::Location;

mod map_location;
pub use map_location::MapLocation;

mod section;
pub use section::Section;

pub struct Tracker {
    root: PathBuf,
    maps: Vec<Map>,
    locations: Vec<Location>,
    variant_uid: VariantUID,
}

impl Tracker {
    pub fn new(root: impl Into<PathBuf>, variant_uid: &VariantUID) -> Self {
        Self {
            root: root.into(),
            maps: Vec::new(),
            locations: Vec::new(),
            variant_uid: variant_uid.clone(),
        }
    }

    pub fn maps(&self) -> &[Map] {
        &self.maps
    }

    pub fn locations(&self) -> &[Location] {
        &self.locations
    }
}

impl UserData for Tracker {
    fn add_fields<'lua, F: UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("ActiveVariantUID", |_, this| {
            Ok(this.variant_uid.as_str().to_owned())
        });
    }

    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut("AddMaps", |_, this, maps_path: String| {
            let maps_path = this.root.join(maps_path);
            let maps = fs::read_to_string(&maps_path)?;
            let maps = maps.strip_prefix(BOM).unwrap_or(&maps);
            let mut maps = serde_hjson::from_str::<Vec<Map>>(maps)
                .with_context(|| eyre!("failed to parse maps json at {maps_path:?}"))
                .map_err(|err| mlua::Error::runtime(format!("{err:?}")))?;

            this.maps.append(&mut maps);

            Ok(())
        });

        methods.add_method_mut("AddItems", |_, this, items_path: String| {
            eprintln!("TODO: Tracker.AddItems");

            Ok(())
        });

        methods.add_method_mut("AddLocations", |_, this, locations_path: String| {
            let locations_path = this.root.join(locations_path);
            let locations = fs::read_to_string(&locations_path)?;
            let locations = locations.strip_prefix(BOM).unwrap_or(&locations);
            let mut locations = serde_hjson::from_str::<Vec<Location>>(locations)
                .with_context(|| eyre!("failed to parse locations json at {locations_path:?}"))
                .map_err(|err| mlua::Error::runtime(format!("{err:?}")))?;

            this.locations.append(&mut locations);

            Ok(())
        });

        methods.add_method_mut("AddLayouts", |_, this, layouts_path: String| {
            eprintln!("TODO: Tracker.AddLayouts");

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
