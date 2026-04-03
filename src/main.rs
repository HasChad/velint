use anyhow::Result;
use winit::event_loop::{ControlFlow, EventLoop};

pub mod app;
pub mod brush_style;
pub mod draw_controller;
pub mod rectangle_style;

use app::*;

fn main() -> Result<()> {
    let mut app = Velint::new();

    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Wait);
    event_loop
        .run_app(&mut app)
        .expect("Couldn't run event loop");
    Ok(())
}
