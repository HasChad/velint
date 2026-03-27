use lyon::path::{Path, Winding};
use lyon::{geom::Box2D, math::point};
use macroquad::prelude::*;

use crate::drawing::{DrawState, Drawable, RectStyle, lyon_ops::*};

impl Drawable for RectStyle {
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

            let rect = Box2D::new(point(p1.x, p1.y), point(p2.x, p2.y));

            builder.add_rectangle(&rect, Winding::Positive);

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

        let rect = Box2D::new(point(p1.x, p1.y), point(p2.x, p2.y));

        builder.add_rectangle(&rect, Winding::Positive);

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
