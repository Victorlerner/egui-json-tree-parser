use std::collections::HashMap;

use eframe::egui::{RichText, Ui};
use egui::{Align, Button, Layout};
use egui_json_tree::{DefaultExpand, JsonTree};
use serde_json::{json, Value};

trait Show {
  fn title(&self) -> &'static str;
  fn show(&mut self, ui: &mut Ui);
}


struct CustomExample {
  title: &'static str,
  input: String,
}

impl CustomExample {
  fn new(title: &'static str) -> Self {
    Self {
      title,
      input: serde_json::to_string_pretty(&json!({"foo": "bar"})).unwrap(),
    }
  }
}

impl Show for CustomExample {
  fn title(&self) -> &'static str {
    self.title
  }

  fn show(&mut self, ui: &mut Ui) {
    ui.label("Enter raw JSON in the text box to see the visualisation below.");


    ui.add_space(ui.spacing().item_spacing.y);
    egui::ScrollArea::vertical()
      .id_source("serial_output")
      .auto_shrink([false; 2])
      .stick_to_bottom(true)
      .max_height(500.0)
      .enable_scrolling(true)
      .show(ui, |ui| {
        ui.add(
          egui::TextEdit::multiline(&mut self.input)
            .code_editor()
            .desired_rows(20)
            .desired_width(f32::INFINITY)
          ,
        );
      });
    let value: serde_json::Result<Value> = serde_json::from_str(&self.input);
    let pretty_string = value
      .as_ref()
      .ok()
      .and_then(|v| serde_json::to_string_pretty(v).ok());

    ui.add_space(ui.spacing().item_spacing.y);
    ui.add_enabled_ui(pretty_string.is_some(), |ui| {
      if ui.button("Beautify").clicked() {
        self.input = pretty_string.unwrap();
      }
    });

    ui.add_space(ui.spacing().item_spacing.y);
    ui.separator();

    match value.as_ref() {
      Ok(value) => {
        JsonTree::new(self.title, value).show(ui);
      }
      Err(err) => {
        ui.label(RichText::new(err.to_string()).color(ui.visuals().error_fg_color));
      }
    };

    ui.add_space(ui.spacing().item_spacing.y);
  }
}


struct DemoApp {
  examples: Vec<Box<dyn Show>>,
  open_example_titles: HashMap<&'static str, bool>,
}

impl Default for DemoApp {
  fn default() -> Self {
    Self {
      examples: vec![
        Box::new(CustomExample::new("Custom Input2")),
      ],
      open_example_titles: HashMap::new(),
    }
  }
}


impl eframe::App for DemoApp {
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    let mut inputJson: String = String::from("");
    let content: String = (0..10).map(|s| s.to_string() + "\n").collect();

    egui::SidePanel::left("left_panel")
      .resizable(true)
      .show(ctx, |ui| {
        /* ui.with_layout(egui::Layout::top_down_justified(egui::Align::LEFT), |ui| {
           for example in self.examples.iter() {
             let is_open = self.open_example_titles.entry(example.title()).or_default();

             ui.toggle_value(is_open, example.title());
           }
         });*/
        ui.label("Enter raw JSON in the text box to see the visualisation below.");
        ui.add_space(ui.spacing().item_spacing.y);

        egui::ScrollArea::vertical()
          .id_source("serial_output")
          .auto_shrink([false; 2])
          .stick_to_bottom(true)
          //  .max_height(280.0)
          .enable_scrolling(true)
          .show(ui, |ui| {
            ui.add(
              // egui::TextEdit::multiline(&mut content.as_str())
              //   .lock_focus(true)
              //   .text_color(egui::Color32::WHITE)
              //   .desired_width(1000.0)
              /*  egui::TextEdit::multiline(&mut inputJson.as_str())
                  .lock_focus(true)
                  .text_color(egui::Color32::WHITE)
                  .desired_width(1000.0)*/

              egui::TextEdit::multiline(&mut content.as_str())
                .code_editor()
                .desired_rows(20)
                .desired_width(f32::INFINITY)
            );
          });

        ui.add_space(ui.spacing().item_spacing.y);
        ui.add_space(ui.spacing().item_spacing.y);
        ui.add_space(ui.spacing().item_spacing.y);
        ui.add_space(ui.spacing().item_spacing.y);
        ui.add_space(ui.spacing().item_spacing.y);
        ui.add_space(ui.spacing().item_spacing.y);
      });

///////////////
    egui::CentralPanel::default().show(ctx, |ui| {
      ui.heading("Hello World!");


      // egui::ScrollArea::vertical()
      //   .id_source("serial_output2")
      //   .auto_shrink([false; 2])
      //   .stick_to_bottom(true)
      //   .enable_scrolling(true)
      //   .show(ui, |ui| {
      //     ui.add(
      //       egui::TextEdit::multiline(&mut content.as_str())
      //         .lock_focus(true)
      //         .text_color(egui::Color32::WHITE)
      //         .desired_width(1000.0)
      //     );
      //   });


      /*  ui.label("Search:");
        let mut text = String::from("test");

        let (text_edit_response, clear_button_response) = ui
          .horizontal(|ui| {
            let text_edit_response = ui.text_edit_singleline(&mut text);
            let clear_button_response = ui.button("Clear");
            (text_edit_response, clear_button_response)
          })
          .inner;


        let value = serde_json::json!({ "foo": "bar", "fizz": [1, 2, 3]});

  // Simple:
        JsonTree::new("simple-tree", &value).show(ui);*/

      for example in self.examples.iter_mut() {
        example.show(ui);
      }
    });

/////////
    /* for example in self.examples.iter_mut() {
       let is_open = self.open_example_titles.entry(example.title()).or_default();

       egui::Window::new(example.title())
         .open(is_open)
         .show(ctx, |ui| example.show(ui));
     }*/
  }

  fn clear_color(&self, visuals: &egui::Visuals) -> [f32; 4] {
    visuals.panel_fill.to_normalized_gamma_f32()
  }
}

fn main() {
  let _ = eframe::run_native(
    "egui-json-tree example",
    eframe::NativeOptions::default(),
    Box::new(|_cc| Box::<DemoApp>::default()),
  );
}