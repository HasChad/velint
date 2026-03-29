use vello::{
    Scene,
    kurbo::{Affine, Rect, Shape, Stroke},
};

use crate::{AppState, ShapeProp};

pub fn rectangle_input(app_state: &mut AppState) {
    if app_state.points.is_empty() {
        app_state.points.push(app_state.mouse_pos);
        app_state.points.push(app_state.mouse_pos);
    } else {
        app_state.points[1] = app_state.mouse_pos;
    }
}

pub fn rectangle_draw(scene: &mut Scene, app_state: &mut AppState) {
    if !app_state.points.is_empty() {
        let p0 = app_state.points[0];
        let p1 = app_state.points[1];

        let rect = Rect::new(p0.x, p0.y, p1.x, p1.y);

        let stroke = Stroke::new(app_state.brush_size as f64);
        scene.stroke(
            &stroke,
            Affine::IDENTITY,
            app_state.brush_color,
            None,
            &rect,
        );
    }
}

pub fn rectangle_complete(app_state: &AppState) -> ShapeProp {
    let p0 = app_state.points[0];
    let p1 = app_state.points[1];

    let path = Rect::new(p0.x, p0.y, p1.x, p1.y).to_path(1.0);

    ShapeProp {
        path: path,
        color: app_state.brush_color,
        size: app_state.brush_size,
        is_fill: false,
    }
}
