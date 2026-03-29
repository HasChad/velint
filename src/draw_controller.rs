use vello::{
    Scene,
    kurbo::{Affine, Circle, Stroke},
    peniko::{Color, Fill},
};

use crate::{AppState, Tool, brush_style::brush_draw, rectangle_style::rectangle_draw};

pub fn prev_shape_draw(scene: &mut Scene, app_state: &mut AppState) {
    if !app_state.shapes.is_empty() {
        for shape in app_state.shapes.iter() {
            if shape.is_fill {
                scene.fill(
                    Fill::NonZero,
                    Affine::IDENTITY,
                    shape.color,
                    None,
                    &shape.path,
                );
            } else {
                let stroke = Stroke::new(shape.size as f64);
                scene.stroke(&stroke, Affine::IDENTITY, shape.color, None, &shape.path);
            }
        }
    }
}

pub fn current_shape_draw(scene: &mut Scene, app_state: &mut AppState) {
    match &app_state.current_tool {
        Tool::Brush => brush_draw(scene, app_state),
        Tool::Rectangle => rectangle_draw(scene, app_state),
    }
}

pub fn cursor_draw(scene: &mut Scene, app_state: &mut AppState) {
    let stroke = Stroke::new(2.0);
    let circle = Circle::new(
        (app_state.mouse_pos.x, app_state.mouse_pos.y),
        app_state.brush_size as f64 / 2.0,
    );
    let circle_fill_color = Color::new([0.7, 0.7, 0.7, 1.]);
    scene.stroke(&stroke, Affine::IDENTITY, circle_fill_color, None, &circle);
}
