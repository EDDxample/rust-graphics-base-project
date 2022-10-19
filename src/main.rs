use std::time::Duration;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let _window = video_subsystem
        .window("title", 800, 600)
        .position_centered()
        .build()
        .expect("could not initialize teh video subsystem");

    let mut event_pump = sdl_context.event_pump()?;

    'running: loop {
        for event in event_pump.poll_event() {
            match event {
                Event::Quit { .. } => {
                    break 'running;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                _ => {}
            }
        }
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60))
    }

    println!("Hello, world!");
    Ok(())
}
