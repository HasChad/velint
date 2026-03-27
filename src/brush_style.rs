use vello::{
    Scene,
    kurbo::{Affine, BezPath, Stroke},
};

use crate::AppState;

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
