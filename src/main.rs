mod window_management;
mod app_loop;
mod rendering;
mod event_handling;

pub(crate) const WIDTH: usize = 80;
pub(crate) const HEIGHT: usize = 40;

fn main() {
    let (sdl_context, mut canvas) = window_management::initialize_window().expect("Failed to initialize window");
    let mut event_pump = sdl_context.event_pump().expect("Failed to get event pump");

    app_loop::run_app_loop(&mut canvas, &mut event_pump);
}
