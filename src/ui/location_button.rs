use chumsky::primitive::Any;
use egui::{popup, Color32, PopupCloseBehavior, Rounding, Sense, Stroke, Ui, Vec2, Widget};
use tracing::trace;

use crate::pack::api::tracker::flat::Location;
use crate::pack::api::tracker::nested::MapLocation;
use crate::pack::api::{AccessibilityLevel, Tracker};
use crate::ui::LocationPopup;

pub struct LocationButton<'a> {
    popup_id: egui::Id,
    location: &'a Location,
    map_location: &'a MapLocation,
    tracker: &'a Tracker,
}

impl<'a> LocationButton<'a> {
    pub fn new(
        ui: &Ui,
        location: &'a Location,
        map_location: &'a MapLocation,
        tracker: &'a Tracker,
    ) -> Self {
        Self {
            popup_id: ui.make_persistent_id((
                &map_location.map,
                &location.name,
                map_location.x,
                map_location.y,
            )),
            location,
            map_location,
            tracker,
        }
    }
}

impl<'a> Widget for LocationButton<'a> {
    fn ui(self, ui: &mut Ui) -> egui::Response {
        let size = Vec2::new(10., 10.);

        let sense = Sense::hover() | Sense::click();
        let (rect, response) = ui.allocate_exact_size(size, sense);

        let popup_id = self.popup_id;
        let mut popup_just_opened = false;

        if response.hovered {
            ui.memory_mut(|mem| mem.open_popup(popup_id));
            popup_just_opened = true;
        };

        let popup_is_open = ui.memory(|mem| mem.is_popup_open(popup_id));

        // TODO: move section level aggregation into function
        let mut accessible = false;
        let mut inaccessible = false;
        let mut sequence_breakable = false;
        let mut checkable = false;
        let mut all_cleared = true;

        for section in &self.location.sections {
            match self.tracker.section_accessibility_level(section) {
                AccessibilityLevel::None => {
                    inaccessible = true;
                    all_cleared = false;
                }
                AccessibilityLevel::Partial => {
                    inaccessible = true;
                    all_cleared = false;
                }
                AccessibilityLevel::Inspect => {
                    checkable = true;
                    all_cleared = false;
                }
                AccessibilityLevel::SequenceBreak => {
                    sequence_breakable = true;
                    all_cleared = false;
                }
                AccessibilityLevel::Normal => {
                    accessible = true;
                    all_cleared = false;
                }
                AccessibilityLevel::Cleared => {}
            }
        }

        let inner_color = if all_cleared {
            Color32::DARK_GRAY
        } else if sequence_breakable {
            Color32::YELLOW
        } else if checkable {
            Color32::BLUE
        } else if accessible && inaccessible {
            Color32::ORANGE
        } else if accessible {
            Color32::GREEN
        } else if inaccessible {
            Color32::RED
        } else {
            Color32::from_rgb(255, 0, 255)
        };

        let outline_color = if popup_is_open {
            Color32::RED
        } else {
            Color32::BLACK
        };

        ui.painter().rect(
            rect,
            Rounding::ZERO,
            inner_color,
            Stroke::new(2., outline_color),
        );

        if popup_is_open {
            let window_fill = &mut ui.style_mut().visuals.window_fill;
            *window_fill = window_fill.gamma_multiply(0.9);

            let popup_response = popup::popup_below_widget(
                ui,
                popup_id,
                &response,
                PopupCloseBehavior::CloseOnClickOutside,
                |ui| {
                    ui.scope(|ui| ui.add(LocationPopup::new(self.location, self.tracker)))
                        .response
                },
            );

            if !popup_just_opened {
                if let Some(popup_response) = popup_response {
                    if let Some(pointer_pos) = ui.ctx().pointer_latest_pos() {
                        let popup_area = popup_response.rect.expand(35.);
                        let hovering = popup_area.contains(pointer_pos);

                        if !hovering {
                            trace!("Closing popup!");
                            ui.memory_mut(|mem| mem.close_popup());
                        }
                    }
                }
            }
        }

        response
    }
}
