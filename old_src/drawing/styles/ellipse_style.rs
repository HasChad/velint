use lyon::geom::{Angle, Vector};
use lyon::math::point;
use lyon::path::{Path, Winding};
use macroquad::prelude::*;

use crate::drawing::{DrawState, Drawable, EllipseStyle, lyon_ops::*};

impl Drawable for EllipseStyle {
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

            builder.add_ellipse(
                point(center.x, center.y),
                Vector::new((p2.x - p1.x) / 2.0, (p2.y - p1.y) / 2.0),
                Angle::zero(),
                Winding::Positive,
            );

            let path = builder.build();

            let lops = LyonOpsFill::new(&path, state.brush_color);

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

        builder.add_ellipse(
            point(center.x, center.y),
            Vector::new((p2.x - p1.x) / 2.0, (p2.y - p1.y) / 2.0),
            Angle::zero(),
            Winding::Positive,
        );

        let path = builder.build();

        let lops = LyonOpsFill::new(&path, state.brush_color);

        let mesh = Mesh {
            vertices: lops.vertices,
            indices: lops.geometry.indices,
            texture: None,
        };

        let last = state.lines.len() - 1;
        state.lines[last].push(mesh);
    }
}
