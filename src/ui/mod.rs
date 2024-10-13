mod location_button;
mod location_popup;

pub use location_button::LocationButton;
pub use location_popup::LocationPopup;

pub mod image {
    use egui::{include_image, ImageSource};

    pub const CLOSED: ImageSource = include_image!("../../assets/closed.png");
    pub const OPEN: ImageSource = include_image!("../../assets/open.png");
}
