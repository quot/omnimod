use std::fs::read_to_string;
use mlua::prelude::*;

pub struct OMAppBase {
    // Members set here
}

impl Default for OMAppBase {
    fn default() -> Self {
        Self {
            // Default values set here
        }
    }
}

impl OMAppBase {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }
}

impl eframe::App for OMAppBase {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self { } = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Side Panel");

            //ui.horizontal(|ui| {
            //    ui.label("Write something: ");
            //    ui.text_edit_singleline(label);
            //});

            //ui.add(egui::Slider::new(value, 0.0..=10.0).text("value"));
            //if ui.button("Increment").clicked() {
            //    *value += 1.0;
            //}

            //ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
            //    ui.horizontal(|ui| {
            //        ui.spacing_mut().item_spacing.x = 0.0;
            //        ui.label("powered by ");
            //        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
            //        ui.label(" and ");
            //        ui.hyperlink_to(
            //            "eframe",
            //            "https://github.com/emilk/egui/tree/master/crates/eframe",
            //        );
            //        ui.label(".");
            //    });
            //});
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            ui.heading("eframe template");
            ui.hyperlink("https://github.com/emilk/eframe_template");
            ui.add(egui::github_link_file!(
                "https://github.com/emilk/eframe_template/blob/master/",
                "Source code."
            ));
            egui::warn_if_debug_build(ui);
        });
    }
}

fn main() -> eframe::Result<()> {
    run_lua_file("/home/deck/Devel/Rust/omnimod/res/hello.lua");

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "om template",
        native_options,
        Box::new(|cc| Box::new(OMAppBase::new(cc))),
        )
}

fn run_lua_file(file_loc: &str) {
    let lua_multiline: String = read_to_string(file_loc).unwrap();
    //println!("{}", lua_multiline);

    let lua = Lua::new();

    match lua.load(&lua_multiline).exec() {
        Err(ex) => panic!("{:?}", ex),
        _ => (),
    };
}
