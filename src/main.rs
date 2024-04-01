use eframe::egui;
use eframe::egui::{Button, FontData, FontDefinitions, FontFamily, FontId};

use eframe::egui::{ TextStyle};
use eframe::egui::FontFamily::{Proportional};
use eframe::egui::TextStyle::{Body, Heading, Small}; // Import TextStyle here

use egui_json_tree::{DefaultExpand, JsonTree, JsonTreeStyle};


fn main() {


    let native_options = eframe::NativeOptions::default();
    eframe::run_native("My egui App", native_options, Box::new(|cc| Box::new(MyEguiApp::new(cc))));



}

#[derive(Default)]
struct MyEguiApp {}




impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.

        Self::default()
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {

        /*let mut style = (*ctx.style()).clone();
        style.text_styles = [
            (Heading, FontId::new(100.0, Proportional)),
        ]
          .into();
        ctx.set_style(style);*/

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello World!");

            let value = serde_json::json!({ "foo": "bar", "fizz": [1, 2, 3]});

// Simple:
            JsonTree::new("simple-tree", &value).show(ui);
        });

    }
}