use std::collections::HashMap;
use std::fs;

use eframe::egui::{RichText, Ui};
use eframe::emath::Vec2;
use egui::{Align, Button, Color32, Layout};
use egui_json_tree::{DefaultExpand, JsonTree, JsonTreeStyle};
use serde_json::{json, Value};

pub enum Message {
  FileOpen(std::path::PathBuf),
  // Other messages
}
#[derive(Default)]
struct MyApp {
  folder_title: String,
  inputJson: String,
  dropped_files: Vec<egui::DroppedFile>,
  picked_path: Option<String>,
  dropped_files_processed: bool,
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
        ui.add_space(ui.spacing().item_spacing.y);
        ui.heading("Enter raw JSON in the text box to see the visualisation below.");
        // ui.label("Enter raw JSON in the text box to see the visualisation below.");
        ui.add_space(ui.spacing().item_spacing.y);

        if ui.button("Open file…").clicked() {
          if let Some(path) = rfd::FileDialog::new().pick_file() {
            self.picked_path = Some(path.display().to_string());
          }
        }

        if let Some(picked_path) = &self.picked_path {
          ui.horizontal(|ui| {
            ui.label("Picked file:");
            ui.monospace(picked_path);
          });
        }
        // Show dropped files (if any):
        if !self.dropped_files.is_empty() {
          ui.group(|ui| {
            ui.label("Dropped files:");


            for file in &self.dropped_files {
              let mut info = if let Some(path) = &file.path {
                path.display().to_string()
              } else if !file.name.is_empty() {
                file.name.clone()
              } else {
                "???".to_owned()
              };

              // Чтение содержимого файла

              /* let content = fs::read_to_string(file.path.as_ref().unwrap());
               println!("Content: {:?}", content);*/

              // Вывод содержимого файла в консоль

              let mut additional_info = vec![];
              if !file.mime.is_empty() {
                additional_info.push(format!("type: {}", file.mime));
              }
              if let Some(bytes) = &file.bytes {
                additional_info.push(format!("{} bytes", bytes.len()));
              }
              if !additional_info.is_empty() {
                info += &format!(" ({})", additional_info.join(", "));
              }

              ui.label(info);
            }
          });
        }

        if !self.dropped_files.is_empty() {
          for file in &self.dropped_files {
            let content = fs::read_to_string(file.path.as_ref().unwrap());
          //  println!("2Content: {:?}", &content);

            let content = content
              .as_ref()
              .ok();

            if let Some(string) = content {
              self.inputJson = String::from(string);
            }
          }
          // Clear dropped files after processing
          self.dropped_files.clear();
        }


        ui.add_enabled_ui(pretty_string.is_some(), |ui| {
          if ui.button("Beautify").clicked() {
            self.inputJson = pretty_string.unwrap();
            println!("Beautified JSON {:?}", self.inputJson);
          }
        });
        if ui.add(egui::Button::new("Clear")).clicked() {
          self.inputJson = "{}".to_string();
        }
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
            )
              //.hovered();
          });
        ui.add_space(ui.spacing().item_spacing.y);
      });

///////////////
    egui::CentralPanel::default()
      .show(ctx, |ui| {
        ui.heading("Hello World!");



        ///
        ui.add_space(ui.spacing().item_spacing.y);
        ui.separator();

        match value.as_ref() {
          Ok(value) => {
            JsonTree::new("99999999999", value)
              .default_expand(DefaultExpand::SearchResults("choice"))
              .show(ui);
          }
          Err(err) => {
            ui.label(RichText::new(err.to_string()).color(ui.visuals().error_fg_color));
          }
        };

        ui.add_space(ui.spacing().item_spacing.y);
      });

    // Collect dropped files:
    ctx.input(|i| {
      if !i.raw.dropped_files.is_empty() {
        self.dropped_files = i.raw.dropped_files.clone();
      }
    });
  }

  fn clear_color(&self, visuals: &egui::Visuals) -> [f32; 4] {
    visuals.panel_fill.to_normalized_gamma_f32()
  }
}

fn main() {
  let options = eframe::NativeOptions {
    viewport: egui::ViewportBuilder::default()
      .with_inner_size([1920.0, 1080.0]) // wide enough for the drag-drop overlay text
      .with_drag_and_drop(true),
    ..Default::default()
  };
  eframe::run_native(
    "Dev Tools",
    options,
    Box::new(|_cc| Box::<MyApp>::default()),
  );
}