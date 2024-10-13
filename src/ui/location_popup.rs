use egui::{Image, Response, ScrollArea, Ui, Vec2, Widget};

use crate::pack::api::tracker::Section;
use crate::ui::image;

pub struct LocationPopup<'a> {
    location_name: &'a str,
    sections: &'a [Section],
}

impl<'a> LocationPopup<'a> {
    pub fn new(location_name: &'a str, sections: &'a [Section]) -> Self {
        Self {
            location_name,
            sections,
        }
    }
}

impl Widget for LocationPopup<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        ScrollArea::vertical()
            .max_height(ui.available_height())
            .show(ui, |ui| {
                ui.set_min_width(150.);
                ui.vertical(|ui| {
                    ui.strong(self.location_name);

                    for section in self.sections {
                        if let Some(name) = &section.name {
                            ui.strong(name);
                        }

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
