use sdl2::event::Event;

pub(crate) fn handle_event(event: Event) {
    match event {
        Event::KeyDown { keycode: Some(keycode), .. } => {
            println!("Key pressed: {:?}", keycode);
        },
        _ => {}
    }
}