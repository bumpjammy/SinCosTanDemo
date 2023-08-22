mod window_management;
mod app_loop;
mod rendering;
mod event_handling;
mod utils;

pub(crate) const WIDTH: i32 = 1600;
pub(crate) const HEIGHT: i32 = 800;

pub(crate) const CIRCLE_CENTRE: (i32, i32) = (200, 600);
pub(crate) const CIRCLE_RADIUS: i32 = 100;

fn main() {
    let (sdl_context, mut canvas) = window_management::initialize_window().expect("Failed to initialize window");
    let mut event_pump = sdl_context.event_pump().expect("Failed to get event pump");

    app_loop::run_app_loop(&mut canvas, &mut event_pump);
}
