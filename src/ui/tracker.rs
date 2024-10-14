use egui::SizeHint;
use egui::TextureOptions;
use egui::Widget;

use egui::{Image, Rect, Vec2};

use crate::pack::{self, Pack};
use crate::ui::LocationButton;

pub struct Tracker {
    pack: Pack,
    current_map: usize,
}

impl Tracker {
    pub fn new(pack: Pack) -> Self {
        Self {
            pack,
            current_map: 0,
        }
    }

    pub fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.pack.api.with_tracker(|tracker| {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.horizontal_wrapped(|ui| {
                    for (i, map) in tracker.maps().iter().enumerate() {
                        ui.selectable_value(&mut self.current_map, i, &map.name);
                    }
                });

                // // Preload all images
                // for map in tracker.maps() {
                //     let map_image_path =
                //         format!("file://{}", self.pack.root.join(&map.img).display());

                //     ImageSource::Uri(map_image_path.into()).load(
                //         ctx,
                //         TextureOptions::default(),
                //         SizeHint::default(),
                //     );
                // }

                if let Some(map) = tracker.maps().get(self.current_map) {
                    let map_image_path =
                        format!("file://{}", self.pack.root.join(&map.img).display());
                    let map_image = Image::new(map_image_path);
                    let map_image_size = map_image
                        .source(ctx)
                        .load(ctx, TextureOptions::default(), SizeHint::default())
                        .map(|texture_poll| texture_poll.size())
                        .unwrap_or(None);

                    let map_image_resp = ui.add(map_image);
                    let map_widget_rect = map_image_resp.rect;

                    Self::add_locations(
                        ctx,
                        ui,
                        tracker,
                        map_widget_rect,
                        map_image_size,
                        self.current_map,
                    );
                }
            });
        });
    }

    fn add_locations(
        ctx: &egui::Context,
        ui: &mut egui::Ui,
        tracker: &pack::api::Tracker,
        map_widget_rect: Rect,
        map_image_size: Option<Vec2>,
        current_map: usize,
    ) {
        let current_map = tracker
            .maps()
            .get(current_map)
            .map(|map| map.name.as_str())
            .unwrap_or_default();

        // let current_map_name = tra
        let Some(map_image_size) = map_image_size else {
            eprintln!("map image size unknown!");
            return;
        };

        let Vec2 {
            x: width,
            y: height,
        } = map_image_size;

        let width = width as f32;
        let height = height as f32;

        for location in tracker.locations() {
            for map_location in &location.map_locations {
                if map_location.map != current_map {
                    continue;
                }

                let x = map_location.x as f32 / width * map_widget_rect.width();
                let y = map_location.y as f32 / height * map_widget_rect.height();

                let button_rect = Rect {
                    min: map_widget_rect.min + Vec2::new(x, y) - Vec2::splat(5.),
                    max: map_widget_rect.min + Vec2::new(x, y) + Vec2::splat(5.),
                };

                let location_button = LocationButton::new(ui, location, map_location);
                let button_response = ui.put(button_rect, location_button);
            }
        }
    }
}
