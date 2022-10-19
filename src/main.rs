use std::time::{Duration, Instant};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::EventPump;

const FRAME_DURATION: Duration = Duration::new(0, 1_000_000_000u32 / 60);

fn main() -> Result<(), String> {
    println!("Hello, world!");

    // create sdl boilerplate
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("title", 800, 600)
        .position_centered()
        .build()
        .expect("could not initialize teh video subsystem");
    let mut event_pump = sdl_context.event_pump()?;

    let mut canvas = window
        .into_canvas()
        .build()
        .map_err(|err| err.to_string())?;

    gameloop(&mut event_pump, &mut canvas)?;
    Ok(())
}

fn render(canvas: &mut WindowCanvas) -> Result<(), String> {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    let pix_w: u32 = 800 / 256 + 1;
    let pix_h: u32 = 600 / 256 + 1;

    for i in 0..=256 {
        for j in 0..=256 {
            canvas.set_draw_color(Color::RGB(i as u8, j as u8, 255));
            canvas
                .fill_rect(Rect::new(i * pix_w as i32, j * pix_h as i32, pix_w, pix_h))
                .unwrap();
        }
    }

    canvas.present();
    Ok(())
}

fn gameloop(event_pump: &mut EventPump, canvas: &mut WindowCanvas) -> Result<(), String> {
    let mut previous_frame = Instant::now();

    'running: loop {
        for event in event_pump.poll_event() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                Event::KeyDown {
                    keycode, keymod, ..
                } => {
                    println!("{:?}, {:?}", keycode.unwrap(), keymod)
                }
                _ => {}
            }
        }

        let timestamp = Instant::now();
        if timestamp >= previous_frame + FRAME_DURATION {
            render(canvas)?;
            previous_frame = timestamp;
        }
    }

    Ok(())
}
