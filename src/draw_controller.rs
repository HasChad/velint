use vello::{
    Scene,
    kurbo::{Affine, Circle, Stroke},
    peniko::Color,
};

use crate::{AppState, Tool, brush_style::brush_draw};

pub fn prev_shape_draw(scene: &mut Scene, app_state: &mut AppState) {
    if !app_state.shapes.is_empty() {
        for shape in app_state.shapes.iter() {
            if shape.is_fill {
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
        Tool::Rectangle => {}
    }

    // ---
}

pub fn cursor_draw(scene: &mut Scene, app_state: &mut AppState) {
    if let Some(mpos) = app_state.mouse_pos {
        let stroke = Stroke::new(2.0);
        let circle = Circle::new((mpos.x, mpos.y), app_state.brush_size as f64 / 2.0);
        let circle_fill_color = Color::new([0.7, 0.7, 0.7, 1.]);
        scene.stroke(&stroke, Affine::IDENTITY, circle_fill_color, None, &circle);
    }
}
