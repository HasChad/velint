use egui_macroquad::egui::{self, Align2, Color32, RichText};

pub struct QuitUI {
    pub visible: bool,
    pub quit_app: bool,
}

impl QuitUI {
    pub fn new() -> Self {
        Self {
            visible: false,
            quit_app: false,
        }
    }

    pub fn ui(&mut self, ctx: &egui::Context) {
        egui::Window::new("Quit?")
            .open(&mut self.visible.clone())
            .title_bar(false)
            .resizable(false)
            .collapsible(false)
            .max_width(50.0)
            .max_height(25.0)
            .anchor(Align2::CENTER_CENTER, (0.0, 0.0))
            .show(ctx, |ui| {
                ui.centered_and_justified(|ui| {
                    let text = RichText::new("Quit?").color(Color32::WHITE);

                    ui.heading(text);

                    ui.horizontal(|ui| {
                        if ui.button("Yes").clicked() {
                            self.quit_app = true
                        }

                        if ui.button("No").clicked() {
                            self.visible = false
                        }
                    });
                })
            });
    }
}
