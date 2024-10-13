mod location_button;
mod location_popup;
mod pack_picker;

pub use location_button::LocationButton;
pub use location_popup::LocationPopup;
pub use pack_picker::PackPicker;

pub mod image {
    use egui::{include_image, ImageSource};

    pub const CLOSED: ImageSource = include_image!("../../assets/closed.png");
    pub const OPEN: ImageSource = include_image!("../../assets/open.png");
}
