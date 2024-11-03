use egui::{Image, Response, RichText, ScrollArea, Ui, Vec2, Widget};

use crate::pack::api::tracker::flat::Location;
use crate::pack::api::Tracker;
use crate::ui::{color_for_accessibility_level, image};

pub struct LocationPopup<'a> {
    location: &'a Location,
    tracker: &'a Tracker,
}

impl<'a> LocationPopup<'a> {
    pub fn new(location: &'a Location, tracker: &'a Tracker) -> Self {
        Self { location, tracker }
    }
}

impl Widget for LocationPopup<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        ScrollArea::vertical()
            .max_height(ui.available_height())
            .show(ui, |ui| {
                ui.set_min_width(150.);
                ui.vertical(|ui| {
                    let first_section_has_a_name = self
                        .location
                        .sections
                        .first()
                        .map(|section| !section.name.is_empty())
                        .unwrap_or(true);

                    if first_section_has_a_name {
                        ui.strong(&self.location.name);
                    }

                    // ui.label(format!("{:#?}", self.location.access_rules));

                    for section in &self.location.sections {
                        let level = self.tracker.section_accessibility_level(section);
                        let color = color_for_accessibility_level(&level);

                        let label = if section.name.is_empty() {
                            &self.location.name
                        } else {
                            &section.name
                        };

                        let label = RichText::new(label).color(color);

                        ui.strong(label);

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
