use lyon::math::point;
use lyon::path::Path;
use macroquad::prelude::*;

use crate::drawing::{ArrowStyle, DrawState, Drawable, lyon_ops::*};

impl Drawable for ArrowStyle {
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
            let p1 = state.current_line[0];
            let p2 = state.current_line[1];
            let size = state.brush_size;

            let dir = if p2 == p1 { p1 } else { p2 - p1 };

            let norm = dir.normalize();
            let mid = p2 - (norm * 5.0 * size);

            let normal = Vec2::new(-dir.y, dir.x).normalize();

            let end1 = mid + normal * 2.0 * size;
            let end2 = mid - normal * 2.0 * size;

            let mut builder = Path::builder();

            builder.begin(point(p1.x, p1.y));
            builder.line_to(point(mid.x, mid.y));
            builder.end(false);

            let path = builder.build();

            let lops = LyonOpsLine::new(&path, state.brush_color, state.brush_size);

            let mesh = Mesh {
                vertices: lops.vertices,
                indices: lops.geometry.indices,
                texture: None,
            };

            draw_mesh(&mesh);

            let mut builder = Path::builder();

            builder.begin(point(p2.x, p2.y));
            builder.line_to(point(end1.x, end1.y));
            builder.line_to(point(end2.x, end2.y));
            builder.end(true);

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

        let p1 = state.current_line[0];
        let p2 = state.current_line[1];
        let size = state.brush_size;

        let dir = if p2 == p1 { p1 } else { p2 - p1 };

        let norm = dir.normalize();
        let mid = p2 - (norm * 5.0 * size);

        let normal = Vec2::new(-dir.y, dir.x).normalize();

        let end1 = mid + normal * 2.0 * size;
        let end2 = mid - normal * 2.0 * size;

        let mut builder = Path::builder();

        builder.begin(point(p1.x, p1.y));
        builder.line_to(point(mid.x, mid.y));
        builder.end(false);

        let path = builder.build();

        let lops = LyonOpsLine::new(&path, state.brush_color, state.brush_size);

        let mesh = Mesh {
            vertices: lops.vertices,
            indices: lops.geometry.indices,
            texture: None,
        };

        let last = state.lines.len() - 1;
        state.lines[last].push(mesh);

        let mut builder = Path::builder();

        builder.begin(point(p2.x, p2.y));
        builder.line_to(point(end1.x, end1.y));
        builder.line_to(point(end2.x, end2.y));
        builder.end(true);

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
