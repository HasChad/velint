use lyon::math::point;
use lyon::path::Path;
use macroquad::prelude::*;

use crate::drawing::{CurveStyle, DrawState, Drawable, lyon_ops::*};

impl Drawable for CurveStyle {
    fn drawing(&self, mouse_pos: Vec2, state: &mut DrawState) {
        if is_mouse_button_pressed(MouseButton::Left) {
            if state.current_line.len() == 3 {
                state.meshing();
                state.current_line.clear();
            } else {
                state.current_line.push(Vec2 {
                    x: mouse_pos.x,
                    y: mouse_pos.y,
                });

                state.current_line.push(Vec2 {
                    x: mouse_pos.x,
                    y: mouse_pos.y,
                });
            }
        };

        if is_mouse_button_down(MouseButton::Left) && !state.current_line.is_empty() {
            if state.current_line.len() == 2 {
                state.current_line[1] = Vec2 {
                    x: mouse_pos.x,
                    y: mouse_pos.y,
                };
            }
        }

        if is_mouse_button_released(MouseButton::Left) {
            if state.current_line.len() == 2 {
                state.current_line.push(Vec2 {
                    x: mouse_pos.x,
                    y: mouse_pos.y,
                });
            }
        }

        if state.current_line.len() == 3 {
            state.current_line[2] = Vec2 {
                x: mouse_pos.x,
                y: mouse_pos.y,
            };
        }
    }

    fn draw_preview(&self, state: &DrawState) {
        if state.current_line.len() > 1 {
            let p1 = state.current_line[0];
            let p2 = state.current_line[1];
            let p3 = if state.current_line.len() == 3 {
                state.current_line[2]
            } else {
                state.current_line[1]
            };

            let mut builder = Path::builder();

            builder.begin(point(p1.x, p1.y));
            builder.quadratic_bezier_to(point(p3.x, p3.y), point(p2.x, p2.y));
            builder.line_to(point(p2.x, p2.y));
            builder.end(false);

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

        let p1 = state.current_line[0];
        let p2 = state.current_line[1];
        let p3 = if state.current_line.len() == 3 {
            state.current_line[2]
        } else {
            state.current_line[1]
        };

        let mut builder = Path::builder();

        builder.begin(point(p1.x, p1.y));
        builder.quadratic_bezier_to(point(p3.x, p3.y), point(p2.x, p2.y));
        builder.line_to(point(p2.x, p2.y));
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
    }
}
