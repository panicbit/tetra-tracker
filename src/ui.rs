mod location_button;
mod location_popup;
mod pack_picker;
mod tracker;

use egui::Color32;
pub use location_button::LocationButton;
pub use location_popup::LocationPopup;
pub use pack_picker::PackPicker;
pub use tracker::Tracker;

use crate::pack::api::AccessibilityLevel;

pub mod image {
    use egui::{include_image, ImageSource};

    pub const CLOSED: ImageSource = include_image!("../assets/closed.png");
    pub const OPEN: ImageSource = include_image!("../assets/open.png");
    pub const LOAD: ImageSource = include_image!("../assets/load.png");
}

fn color_for_accessibility_level(level: &AccessibilityLevel) -> Color32 {
    match level {
        AccessibilityLevel::None => Color32::RED,
        AccessibilityLevel::Partial => Color32::ORANGE,
        AccessibilityLevel::Inspect => Color32::BLUE,
        AccessibilityLevel::SequenceBreak => Color32::YELLOW,
        AccessibilityLevel::Normal => Color32::GREEN,
        AccessibilityLevel::Cleared => Color32::DARK_GRAY,
    }
}
