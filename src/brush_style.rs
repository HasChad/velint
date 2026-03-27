use vello::{
    Scene,
    kurbo::{Affine, BezPath, Stroke},
};

use crate::{AppState, ShapeProp, Shapes};

pub fn brush_draw(scene: &mut Scene, app_state: &mut AppState) {
    if !app_state.points.is_empty() {
        let mut bez_path = BezPath::new();
        bez_path.move_to(app_state.points[0]);

        for point in app_state.points.iter_mut() {
            bez_path.line_to(*point);
        }

        let stroke = Stroke::new(app_state.brush_size as f64);
        scene.stroke(
            &stroke,
            Affine::IDENTITY,
            app_state.brush_color,
            None,
            &bez_path,
        );
    }
}

pub fn brush_complete(app_state: &AppState) -> Shapes {
    let mut path = BezPath::new();

    if !app_state.points.is_empty() {
        path.move_to(app_state.points[0]);

        for point in app_state.points.iter() {
            path.line_to(*point);
        }
    }

    Shapes::Brush(ShapeProp {
        path: path,
        color: app_state.brush_color,
        size: app_state.brush_size,
    })
}
