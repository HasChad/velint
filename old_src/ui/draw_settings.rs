use egui_macroquad::egui::{self, Align2, Color32, RichText, Slider};
use macroquad::time::get_fps;

use crate::drawing::{DrawState, DrawStyle};

pub struct DrawSettings {
    pub sides: u32,
    pub rotation: f32,
}

impl DrawSettings {
    pub fn new() -> Self {
        Self {
            sides: 6,
            rotation: 0.0,
        }
    }

    pub fn ui(&mut self, ctx: &egui::Context, state: &mut DrawState) {
        egui::Window::new("Settings")
            .open(&mut true)
            .max_width(100.0)
            .resizable(false)
            .anchor(Align2::LEFT_TOP, (5.0, 5.0))
            .frame(egui::Frame::window(&egui::Style::default()))
            .show(ctx, |ui| {
                egui::Grid::new("poly_grid")
                    .num_columns(1)
                    .spacing([5.0, 5.0])
                    .striped(true)
                    .show(ui, |ui| {
                        ui.label(format!("fps: {:.0}", get_fps()));
                        ui.end_row();

                        ui.add(
                            Slider::new(&mut state.brush_size, 1.0..=30.0)
                                .trailing_fill(true)
                                .step_by(0.1)
                                .text("Brush Size")
                                .text_color(Color32::WHITE),
                        );
                        ui.end_row();

                        let mut egui_color = egui::Color32::WHITE;

                        egui_color[0] = 255.min((state.brush_color.r * 255.0) as u8);
                        egui_color[1] = 255.min((state.brush_color.g * 255.0) as u8);
                        egui_color[2] = 255.min((state.brush_color.b * 255.0) as u8);
                        egui_color[3] = 255.min((state.brush_color.a * 255.0) as u8);

                        ui.horizontal(|ui| {
                            ui.label(RichText::new("Color:").color(Color32::WHITE));
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                if ui.color_edit_button_srgba(&mut egui_color).changed() {
                                    state.brush_color.a = egui_color.a() as f32 / 255.0;
                                    state.brush_color.r = egui_color.r() as f32 / 255.0;
                                    state.brush_color.g = egui_color.g() as f32 / 255.0;
                                    state.brush_color.b = egui_color.b() as f32 / 255.0;
                                }
                            })
                        });
                        ui.end_row();

                        let mut egui_color = egui::Color32::WHITE;

                        egui_color[0] = 255.min((state.bg_color.r * 255.0) as u8);
                        egui_color[1] = 255.min((state.bg_color.g * 255.0) as u8);
                        egui_color[2] = 255.min((state.bg_color.b * 255.0) as u8);
                        egui_color[3] = 255.min((state.bg_color.a * 255.0) as u8);

                        ui.horizontal(|ui| {
                            ui.label(RichText::new("BG Color:").color(Color32::WHITE));
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    if ui.color_edit_button_srgba(&mut egui_color).changed() {
                                        state.bg_color.a = egui_color.a() as f32 / 255.0;
                                        state.bg_color.r = egui_color.r() as f32 / 255.0;
                                        state.bg_color.g = egui_color.g() as f32 / 255.0;
                                        state.bg_color.b = egui_color.b() as f32 / 255.0;
                                    }
                                },
                            );
                        });
                        ui.end_row();

                        if state.style == DrawStyle::Poly || state.style == DrawStyle::PolyO {
                            ui.heading("- Polygon Settings -");
                            ui.end_row();

                            ui.add(
                                Slider::new(&mut state.poly_settings.sides, 3..=20)
                                    .trailing_fill(true)
                                    .step_by(0.1)
                                    .text("Edge Count")
                                    .text_color(Color32::WHITE),
                            );
                            ui.end_row();

                            ui.add(
                                Slider::new(&mut state.poly_settings.rotation, 0.0..=360.0)
                                    .trailing_fill(true)
                                    .step_by(0.1)
                                    .text("Rotation")
                                    .text_color(Color32::WHITE),
                            );
                            ui.end_row();
                        }
                    })
            });
    }
}
