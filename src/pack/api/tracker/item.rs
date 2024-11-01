use crate::util::{const_bool, const_i32, option_value_or_string, string_list, value_or_string};
use hex_color::HexColor;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Item {
    pub name: Option<String>,
    /// Code identifier(s) of this item. Multiple values are comma seperated.
    #[serde(default, with = "string_list")]
    pub codes: Vec<String>,
    #[serde(flatten)]
    pub variant: Variant,
    // TODO: overlay_align
    // TODO: capturable
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Variant {
    Static(Static),
    Progressive(Progressive),
    Toggle(Toggle),
    Consumable(Consumable),
    ProgressiveToggle(ProgressiveToggle),
    CompositeToggle(CompositeToggle),
    ToggleBadged(ToggleBadged),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Static {
    /// How to display the item.
    #[serde(flatten)]
    pub display: Display,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Progressive {
    /// Stages of the progressive item.
    pub stages: Vec<Stage>,
    /// Automatically addes a `"off"` stage if set to true.
    #[serde(default = "const_bool::<true>", deserialize_with = "value_or_string")]
    pub allow_disabled: bool,
    /// Initital stage index for the progressive item. Zero indexed.
    #[serde(default, deserialize_with = "value_or_string")]
    pub initial_stage_idx: usize,
    /// Allows looping through the stages.
    #[serde(default = "const_bool::<true>", deserialize_with = "value_or_string")]
    pub r#loop: bool,
}

/// Stage of a progressive item
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Stage {
    /// Readable name of the stage for tooltip.
    pub name: Option<String>,
    /// How to display the item.
    #[serde(flatten)]
    pub display: Display,
    /// Code identifier(s) of this item.
    /// Multiple values are comma seperated.
    #[serde(default, with = "string_list")]
    pub codes: Vec<String>,
    /// Secondary code identifier(s) of this item.
    /// Multiple values are comma seperated.
    /// Unused at the moment.
    #[serde(default, with = "string_list")]
    pub secondary_codes: Vec<String>,
    /// If set to true, stages will provide for the codes
    /// of the previous stages as well.
    #[serde(default = "const_bool::<true>", deserialize_with = "value_or_string")]
    pub inherit_codes: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Toggle {
    /// How to display the item.
    #[serde(flatten)]
    pub display: Display,
    /// Precollected if true.
    #[serde(default)]
    pub initial_active_state: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Consumable {
    /// How to display the item.
    #[serde(flatten)]
    pub display: Display,
    /// Minimum quantity of the consumable. Inclusive.
    #[serde(default, deserialize_with = "value_or_string")]
    pub min_quantity: i32,
    /// Maximum quantity of the consumable. Inclusive.
    #[serde(default, deserialize_with = "value_or_string")]
    pub max_quantity: i32,
    /// Amount to increase the quantity by on left-click.
    #[serde(default = "const_i32::<1>", deserialize_with = "value_or_string")]
    pub increment: i32,
    /// Amount to decrease the quantity by on right-click.
    #[serde(default = "const_i32::<1>", deserialize_with = "value_or_string")]
    pub decrement: i32,
    /// Initial quantity of the consumable.
    #[serde(default)]
    pub initial_quantity: i32,
    /// Background color of the overlay text displaying the quantity.
    pub overlay_background: Option<HexColor>,
    #[serde(default, deserialize_with = "option_value_or_string")]
    /// Font size of the overlay text displaying the quantity.
    /// Mutually exclusive with `bagde_font_size`.
    pub overlay_font_size: Option<u32>,
    /// Font size of the overlay text displaying the quantity.
    /// Mutually exclusive with `overlay_font_size`.
    #[serde(default, deserialize_with = "option_value_or_string")]
    pub badge_font_size: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProgressiveToggle {
    /// Stages of the progressive item.
    pub stages: Vec<Stage>,
    /// Initital stage index for the progressive item. Zero indexed.
    #[serde(default, deserialize_with = "value_or_string")]
    pub initial_stage_idx: usize,
    /// Allows looping through the stages.
    #[serde(default = "const_bool::<true>", deserialize_with = "value_or_string")]
    pub r#loop: bool,
    /// Precollected if true.
    #[serde(default)]
    pub initial_active_state: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum CompositeToggle {
    Simple(SimpleCompositeToggle),
    Complex(ComplexCompositeToggle),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SimpleCompositeToggle {
    /// Code identifier of the left item.
    pub item_left: String,
    /// Code identifier of the right item.
    pub item_right: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ComplexCompositeToggle {
    /// Array of images and states (up to 4, on/off for each item)
    pub images: Vec<CompositeToggleImage>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CompositeToggleImage {
    /// How to display the item.
    #[serde(flatten)]
    pub display: Display,
    /// State of left item.
    #[serde(deserialize_with = "value_or_string")]
    pub left: bool,
    #[serde(deserialize_with = "value_or_string")]
    /// State of right item.
    pub right: bool,
    /// Code identifier(s) of this item.
    /// Multiple values are comma seperated.
    #[serde(default, with = "string_list")]
    pub codes: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ToggleBadged {
    /// How to display the item.
    #[serde(flatten)]
    pub display: Display,
    /// Code identifier for the base item, that this item should overlay.
    base_item: Option<String>,
    /// Precollected overlay if true.
    #[serde(default)]
    pub initial_active_state: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Display {
    /// Image to display.
    pub img: String,
    /// Image to display when disabled.
    pub disabled_img: Option<String>,
    /// Modifiers to apply to the image.
    /// Multiple values are comma seperated.
    pub img_mods: Option<String>,
    /// Modifiers to apply to the image when disabled.
    /// Multiple values are comma seperated.
    pub disabled_img_mods: Option<String>,
}
