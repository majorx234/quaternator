use eframe::{egui, egui_glow, glow};

use egui::mutex::Mutex;
use std::sync::Arc;

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([400.0, 400.0]),
        multisampling: 4,
        renderer: eframe::Renderer::Glow,
        ..Default::default()
    };
    eframe::run_native(
        "Custom 3D painting in eframe using glow",
        options,
        Box::new(|cc| Ok(Box::new(MyApp::new(cc)))),
    )
}
struct MyApp {
    data: u32,
}


impl MyApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        //let gl = cc.gl.as_ref().expect("You need to run eframe with the glow backend");
        Self {
            data: 0,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 0.0;
                ui.label("Cube exmple ");
                ui.label(" (OpenGL).");
            });
        });
    }
}
