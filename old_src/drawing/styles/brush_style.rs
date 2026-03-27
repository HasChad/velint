use lyon::math::point;
use lyon::path::Path;
use macroquad::prelude::*;

use crate::drawing::{
    BrushStyle, DrawState, Drawable, line_smoothing::line_smoothing, lyon_ops::*,
};

impl Drawable for BrushStyle {
    fn drawing(&self, mouse_pos: Vec2, state: &mut DrawState) {
        if is_mouse_button_pressed(MouseButton::Left) {
            state.current_line.push(Vec2 {
                x: mouse_pos.x,
                y: mouse_pos.y,
            });
        };

        if is_mouse_button_down(MouseButton::Left) && !state.current_line.is_empty() {
            if mouse_delta_position().x != 0.0 || mouse_delta_position().y != 0.0 {
                state.current_line.push(Vec2 {
                    x: mouse_pos.x,
                    y: mouse_pos.y,
                });
            }
        }

        if is_mouse_button_released(MouseButton::Left) {
            if state.current_line.len() > 1 {
                if state.current_line.len() > 2 {
                    line_smoothing(&mut state.current_line);
                }
                state.meshing();
            }

            state.current_line.clear();
        }
    }

    fn draw_preview(&self, state: &DrawState) {
        let mut prev_last: Option<&Vec2> = None;

        for line_chunk in state.current_line.chunks(350) {
            let mut builder = Path::builder();
            let mut raw_points = vec![];

            if let Some(prev) = prev_last {
                raw_points.push(point(prev.x, prev.y));
            }

            for stroke in line_chunk.iter() {
                raw_points.push(point(stroke.x, stroke.y));
            }

            prev_last = line_chunk.last();

            for (i, point) in raw_points.iter().enumerate() {
                if i == 0 {
                    builder.begin(*point);
                } else {
                    builder.line_to(*point);
                }

                if i == raw_points.len() - 1 {
                    builder.end(false);
                }
            }

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
        let mut prev_last: Option<&Vec2> = None;
        state.lines.push(vec![]);

        for line_chunk in state.current_line.chunks(350) {
            let mut builder = Path::builder();
            let mut raw_points = vec![];

            if let Some(prev) = prev_last {
                raw_points.push(point(prev.x, prev.y));
            }

            for stroke in line_chunk.iter() {
                raw_points.push(point(stroke.x, stroke.y));
            }

            prev_last = line_chunk.last();

            for (i, point) in raw_points.iter().enumerate() {
                if i == 0 {
                    builder.begin(*point);
                    continue;
                }

                builder.line_to(*point);

                if i == raw_points.len() - 1 {
                    builder.end(false);
                }
            }

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
}
