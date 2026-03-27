use egui_macroquad::egui::{self, Align2};

use crate::drawing::{DrawState, DrawStyle};

pub struct StyleSettings;

impl StyleSettings {
    pub fn new() -> Self {
        Self
    }

    pub fn ui(&mut self, ctx: &egui::Context, state: &mut DrawState) {
        egui::Window::new("Options")
            .title_bar(false)
            .resizable(false)
            .anchor(Align2::RIGHT_TOP, (-5.0, 5.0))
            .show(ctx, |ui| {
                egui::Grid::new("my_grid")
                    .num_columns(2)
                    .spacing([5.0, 5.0])
                    .min_col_width(60.0)
                    .striped(true)
                    .show(ui, |ui| {
                        ui.radio_value(&mut state.style, DrawStyle::Brush, "Brush");
                        ui.end_row();
                        ui.radio_value(&mut state.style, DrawStyle::SBrush, "SBrush");
                        ui.end_row();
                        ui.radio_value(&mut state.style, DrawStyle::Line, "Line");
                        ui.end_row();
                        ui.radio_value(&mut state.style, DrawStyle::Curve, "Curve");
                        ui.end_row();
                        ui.radio_value(&mut state.style, DrawStyle::Arrow, "Arrow");
                        ui.end_row();
                        ui.radio_value(&mut state.style, DrawStyle::Rect, "Rect");
                        ui.end_row();
                        ui.radio_value(&mut state.style, DrawStyle::RectO, "Rect O");
                        ui.end_row();
                        ui.radio_value(&mut state.style, DrawStyle::Circle, "Circle");
                        ui.end_row();
                        ui.radio_value(&mut state.style, DrawStyle::CircleO, "Circle O");
                        ui.end_row();
                        ui.radio_value(&mut state.style, DrawStyle::Ellipse, "Ellipse");
                        ui.end_row();
                        ui.radio_value(&mut state.style, DrawStyle::EllipseO, "Ellipse O");
                        ui.end_row();
                        ui.radio_value(&mut state.style, DrawStyle::Poly, "Poly");
                        ui.end_row();
                        ui.radio_value(&mut state.style, DrawStyle::PolyO, "Poly O");
                    })
            });
    }
}
