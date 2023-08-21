use sdl2::EventPump;
use sdl2::render::WindowCanvas;
use sdl2::sys::SDL_Renderer;
use crate::{event_handling, rendering};

pub(crate) fn run_app_loop(canvas: &mut WindowCanvas, event_pump: &mut EventPump) {
    let mut frame = 0;
    let mut sine_points = Vec::new();
    let mut cosine_points = Vec::new();
    let mut tangent_points = Vec::new();

    'running: loop {
        rendering::render(canvas, frame, &mut sine_points, &mut cosine_points, &mut tangent_points);

        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} |
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Escape), .. } => {
                    break 'running;
                },
                _ => { event_handling::handle_event(event) }
            }
        }

        frame += 1;
        std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 60));
    }
}