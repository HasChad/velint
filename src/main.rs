#![windows_subsystem = "windows"]

use macroquad::prelude::*;

mod app_settings;
mod drawing;
mod ui;

use crate::{drawing::DrawState, ui::ui::UI};
use app_settings::*;

#[macroquad::main(window_conf)]
async fn main() {
    let mut camera = Camera2D {
        zoom: vec2(2. / screen_width(), 2. / screen_height()),
        ..Default::default()
    };
    let mut zoomer = ZOOM_DEFAULT;

    let mut draw_state = DrawState::new();
    let mut ui = UI::new();

    'app: loop {
        camera_fixer(&mut camera, &mut zoomer);
        let world_mpos = camera.screen_to_world(Vec2 {
            x: mouse_position().0,
            y: mouse_position().1,
        });

        if is_key_pressed(KeyCode::Escape) {
            ui.quit_ui.visible = true;
        }
        if ui.quit_ui.quit_app {
            break 'app;
        }

        if draw_state.can_draw {
            draw_state.drawing(world_mpos);
        }

        draw_state.inputs();

        // ! draw
        clear_background(draw_state.bg_color);
        set_camera(&camera);

        ui.render_ui(&mut draw_state);

        draw_state.line_render();
        draw_state.current_line_render();

        draw_circle_lines(
            world_mpos.x,
            world_mpos.y,
            draw_state.brush_size / 2.0 - 1.0,
            1.0,
            WHITE,
        );

        // draw_dashed_rectangle(0.0, 0.0, 100.0, 100.0, WHITE);

        egui_macroquad::draw();

        next_frame().await
    }
}
/*
fn draw_dashed_rectangle(x: f32, y: f32, w: f32, h: f32, color: Color) {
    let line_len = 10;
    let thick = 2.0;

    for p in (0..=(w as i32)).step_by(line_len * 2) {
        draw_line(
            x + p as f32,
            y,
            (p + line_len as i32) as f32,
            y,
            thick,
            color,
        );

        draw_line(
            x + p as f32,
            y + h,
            (p + line_len as i32) as f32,
            y + h,
            thick,
            color,
        );
    }

    for p in (0..=(h as i32)).step_by(line_len * 2) {
        draw_line(
            x,
            y + p as f32,
            x,
            (p + line_len as i32) as f32,
            thick,
            color,
        );

        draw_line(
            x + w,
            y + p as f32,
            x + w,
            (p + line_len as i32) as f32,
            thick,
            color,
        );
    }
}
*/
