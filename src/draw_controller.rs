use vello::{
    Scene,
    kurbo::{Affine, BezPath, Circle, Stroke},
    peniko::Color,
};

use crate::{AppState, Shapes, brush_style::brush_draw};

pub fn prev_shape_draw(scene: &mut Scene, app_state: &mut AppState) {
    if !app_state.shapes.is_empty() {
        for shape in app_state.shapes.iter() {
            match shape {
                Shapes::Brush(points) => {
                    if !points.is_empty() {
                        let mut bez_path = BezPath::new();
                        bez_path.move_to(points[0]);

                        for point in points {
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
                Shapes::Rectangle(_points) => todo!(),
            }
        }
    }
}

pub fn current_shape_draw(scene: &mut Scene, app_state: &mut AppState) {
    match &app_state.current_tool {
        Shapes::Brush(_) => brush_draw(scene, app_state),
        Shapes::Rectangle(_) => {}
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
