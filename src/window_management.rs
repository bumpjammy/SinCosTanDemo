use sdl2::render::WindowCanvas;
use sdl2::sys::{SDL_CreateRenderer, SDL_GetWindowSurface, SDL_Renderer};
use crate::{HEIGHT, WIDTH};

pub(crate) fn initialize_window() -> Result<(sdl2::Sdl, WindowCanvas), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("Sin Cos Tan", (WIDTH * 20) as u32, (HEIGHT * 20) as u32)
        .position(0, 0)
        .build()
        .expect("Failed to initialize");

    let canvas = window.into_canvas().build()
        .expect("Failed to create canvas");

    Ok((sdl_context, canvas))
}