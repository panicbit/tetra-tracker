use egui::{Image, Response, ScrollArea, Ui, Vec2, Widget};

use crate::pack::api::tracker::Location;
use crate::ui::image;

pub struct LocationPopup<'a> {
    location: &'a Location,
}

impl<'a> LocationPopup<'a> {
    pub fn new(location: &'a Location) -> Self {
        Self { location }
    }
}

impl Widget for LocationPopup<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        ScrollArea::vertical()
            .max_height(ui.available_height())
            .show(ui, |ui| {
                ui.set_min_width(150.);
                ui.vertical(|ui| {
                    ui.strong(&self.location.name);
                    // ui.label(format!("{:#?}", self.location.access_rules));

                    for section in &self.location.sections {
                        if let Some(name) = &section.name {
                            ui.strong(name);
                        }

                        // ui.label(format!("{:#?}", section.access_rules));

                        ui.add(
                            Image::new(image::CLOSED)
                                .max_size(Vec2::splat(25.))
                                .fit_to_original_size(1.),
                        );
                    }
                })
            })
            .inner
            .response
    }
}
