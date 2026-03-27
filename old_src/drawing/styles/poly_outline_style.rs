use lyon::math::point;
use lyon::path::{Path, Polygon};
use macroquad::prelude::*;

use crate::drawing::{DrawState, Drawable, PolyOStyle, lyon_ops::*};

impl Drawable for PolyOStyle {
    fn drawing(&self, mouse_pos: Vec2, state: &mut DrawState) {
        if is_mouse_button_pressed(MouseButton::Left) {
            state.current_line.push(Vec2 {
                x: mouse_pos.x,
                y: mouse_pos.y,
            });

            state.current_line.push(Vec2 {
                x: mouse_pos.x,
                y: mouse_pos.y,
            });
        };

        if is_mouse_button_down(MouseButton::Left) && !state.current_line.is_empty() {
            state.current_line[1] = Vec2 {
                x: mouse_pos.x,
                y: mouse_pos.y,
            };
        }

        if is_mouse_button_released(MouseButton::Left) {
            if state.current_line.len() > 1 {
                state.meshing();
            }

            state.current_line.clear();
        }
    }

    fn draw_preview(&self, state: &DrawState) {
        if state.current_line.len() == 2 {
            let mut builder = Path::builder();

            let p1 = state.current_line[0];
            let p2 = state.current_line[1];

            let center = (p1 + p2) * 0.5;

            let sides = state.poly_settings.sides;
            let rot = state.poly_settings.rotation.to_radians();

            let radius = if (p2.x - p1.x).abs() < (p2.y - p1.y).abs() {
                (p2.x - p1.x).abs() * 0.5
            } else {
                (p2.y - p1.y).abs() * 0.5
            };
            let mut points = vec![];

            for i in 0..=sides {
                let rx = (i as f32 / sides as f32 * std::f32::consts::PI * 2. + rot).sin();
                let ry = -(i as f32 / sides as f32 * std::f32::consts::PI * 2. + rot).cos();

                let poly_p = point(center.x + radius * rx, center.y + radius * ry);

                points.push(poly_p);
            }

            builder.add_polygon(Polygon {
                points: &points,
                closed: true,
            });

            let path = builder.build();

            let lops = LyonOpsLine::new(&path, state.brush_color, state.brush_size);

            let mesh = Mesh {
                vertices: lops.vertices,
                indices: lops.geometry.indices,
                texture: None,
            };

            draw_mesh(&mesh);
        }
    }

    fn mesh(&self, state: &mut DrawState) {
        state.lines.push(vec![]);

        let mut builder = Path::builder();

        let p1 = state.current_line[0];
        let p2 = state.current_line[1];

        let center = (p1 + p2) * 0.5;

        let sides = state.poly_settings.sides;
        let rot = state.poly_settings.rotation.to_radians();

        let radius = if (p2.x - p1.x).abs() < (p2.y - p1.y).abs() {
            (p2.x - p1.x).abs() * 0.5
        } else {
            (p2.y - p1.y).abs() * 0.5
        };
        let mut points = vec![];

        for i in 0..=sides {
            let rx = (i as f32 / sides as f32 * std::f32::consts::PI * 2. + rot).sin();
            let ry = -(i as f32 / sides as f32 * std::f32::consts::PI * 2. + rot).cos();

            let poly_p = point(center.x + radius * rx, center.y + radius * ry);

            points.push(poly_p);
        }

        builder.add_polygon(Polygon {
            points: &points,
            closed: true,
        });

        let path = builder.build();

        let lops = LyonOpsLine::new(&path, state.brush_color, state.brush_size);

        let mesh = Mesh {
            vertices: lops.vertices,
            indices: lops.geometry.indices,
            texture: None,
        };

        let last = state.lines.len() - 1;
        state.lines[last].push(mesh);
    }
}
