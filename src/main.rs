use std::collections::HashMap;

use eframe::egui::{RichText, Ui};
use egui::{Align, Button, Layout};
use egui_json_tree::{DefaultExpand, JsonTree};
use serde_json::{json, Value};

#[derive(Default)]
struct MyApp {
  folder_title: String,
  inputJson: String,
}

impl MyApp {
  fn new(cc: &eframe::CreationContext<'_>) -> Self {
    Self {
      folder_title: "Add Folder".to_string(), // Initialize the folder title here
      inputJson: serde_json::to_string(&json!({"foo": "bar"})).unwrap(),
    }
  }
}
impl eframe::App for MyApp {
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    //let content: String = (0..10).map(|s| s.to_string() + "\n").collect();
    let value: serde_json::Result<Value> = serde_json::from_str(&mut self.inputJson);
    let pretty_string = value
      .as_ref()
      .ok()
      .and_then(|v| serde_json::to_string_pretty(v).ok());

    egui::SidePanel::left("left_panel")
      .resizable(true)
      .min_width(900.0)
      .show(ctx, |ui| {

     //   let response = ui.add(egui::TextEdit::singleline(&mut self.folder_title));
       // if response.changed() {}

        ui.add_space(ui.spacing().item_spacing.y);
        ui.heading("Enter raw JSON in the text box to see the visualisation below.");
        // ui.label("Enter raw JSON in the text box to see the visualisation below.");
        ui.add_space(ui.spacing().item_spacing.y);

        ui.add_enabled_ui(pretty_string.is_some(), |ui| {
          if ui.button("Beautify").clicked() {
            self.inputJson = pretty_string.unwrap();
            println!("Beautified JSON {:?}", self.inputJson);
          }
        });
        if ui.add(egui::Button::new("Clear")).clicked() {
          self.inputJson = "{}".to_string();
        }


       /* ui.add_enabled_ui(pretty_string.is_some(), |ui| {
          if ui.button("Clear").clicked() {
            self.inputJson = "{}".to_string();
            println!("Beautified JSON {:?}", self.inputJson);
          }
        });*/


        ui.add_space(ui.spacing().item_spacing.y);
        egui::ScrollArea::vertical()
          .id_source("serial_output")
          .auto_shrink([false; 2])
          .stick_to_bottom(true)
          .max_height(500.0)
          .enable_scrolling(true)
          .show(ui, |ui| {
            ui.add(
              egui::TextEdit::multiline(&mut self.inputJson)
                .code_editor()
                .desired_rows(20)
                .desired_width(f32::INFINITY)
              ,
            );
          });
        ui.add_space(ui.spacing().item_spacing.y);
      });

///////////////
    egui::CentralPanel::default().show(ctx, |ui| {
      ui.heading("Hello World!");

      ui.add_space(ui.spacing().item_spacing.y);
      ui.separator();

      match value.as_ref() {
        Ok(value) => {
          JsonTree::new("99999999999", value).show(ui);
        }
        Err(err) => {
          ui.label(RichText::new(err.to_string()).color(ui.visuals().error_fg_color));
        }
      };

      ui.add_space(ui.spacing().item_spacing.y);
    });
  }

  fn clear_color(&self, visuals: &egui::Visuals) -> [f32; 4] {
    visuals.panel_fill.to_normalized_gamma_f32()
  }
}

fn main() {
  let _ = eframe::run_native(
    "egui-json-tree example",
    eframe::NativeOptions::default(),
    Box::new(|_cc| Box::<MyApp>::default()),
  );
}