use egui::{Button, Pos2, Rect, Vec2};
use tetra_tracker::pack::Pack;

fn main() {
    let lua = mlua::Lua::new();
    lua.load(r#"print("hello from lua!")"#).exec().unwrap();
    let pack = Pack::load("packs/ittledew2-poptracker").unwrap();
    println!("{:#?}", pack.manifest);

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "My egui App",
        native_options,
        Box::new(|cc| Ok(Box::new(MyEguiApp::new(cc)))),
    );
}

#[derive(Default)]
struct MyEguiApp {}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        egui_extras::install_image_loaders(&cc.egui_ctx);

        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello World!");
            let map = ui.image("file://packs/ittledew2-poptracker/images/maps/overworld.png");

            let button = Button::new("Hi");
            let button_rect = Rect {
                min: map.rect.min,
                max: map.rect.min + Vec2::new(20., 20.),
            };
            ui.put(button_rect, button);
        });
    }
}
