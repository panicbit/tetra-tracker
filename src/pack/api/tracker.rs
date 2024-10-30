use std::path::PathBuf;
use std::{fs, iter};

use eyre::{eyre, Context};
use mlua::{UserData, UserDataFields, UserDataMethods};
use tracing::{debug, debug_span, warn};

use crate::pack::VariantUID;
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

    pub fn locations_recursive(&self) -> impl Iterator<Item = &Location> {
        self.locations
            .iter()
            .flat_map(|location| iter::once(location).chain(location.child_locations_recursive()))
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
            let maps = maps.strip_prefix(BOM).unwrap_or(&maps);
            let mut maps = serde_hjson::from_str::<Vec<Map>>(maps)
                .with_context(|| eyre!("failed to parse maps json at {maps_path:?}"))
                .map_err(|err| mlua::Error::runtime(format!("{err:?}")))?;

            this.maps.append(&mut maps);

            Ok(())
        });

        methods.add_method_mut("AddItems", |_, _this, _items_pathh: String| {
            let _span = debug_span!("Tracker::AddItems").entered();
            warn!("TODO: implement");

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
