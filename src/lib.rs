use std::time::{Duration, Instant};

use sdl2::{event::Event, keyboard::Keycode, render::WindowCanvas};

/// Implement this trait and call `GameEngine::run()`.
pub trait GameEngine {
    /// Handles Event::{MouseButtonDown, MouseButtonUp, MouseMotion, MouseWheel}.
    fn handle_mouse(&mut self, _event: Event) {}

    /// Handles Event::{KeyDown, KeyUp}.
    fn handle_key(&mut self, _event: Event) {}

    /// Updates game display.
    fn render(&mut self, _canvas: &mut WindowCanvas) {}

    /// Updates game logic.
    fn tick(&mut self) {}

    /// Basic window and game configuration.
    fn config(&mut self) -> (&str, u32, u32, u32, u32) {
        const TITLE: &str = "My Game";
        const WIDTH: u32 = 500;
        const HEIGHT: u32 = 500;
        const FPS: u32 = 60;
        const TPS: u32 = 20;

        (TITLE, WIDTH, HEIGHT, FPS, TPS)
    }

    /// Game's main loop, you shouldn't reimplement this function.
    fn run(&mut self) -> Result<(), String> {
        let (title, width, height, fps, tps) = self.config();

        if tps == 0 || tps == 0 {
            return Err("FPS and TPS cannot be 0.".to_string());
        }

        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window(title, width, height)
            .position_centered()
            .build()
            .expect("Could not initialize the video subsystem.");

        let mut canvas = window.into_canvas().build().unwrap();
        let mut event_pump = sdl_context.event_pump().unwrap();

        let mut previous_frame = Instant::now();
        let frame_duration = Duration::new(0, 1_000_000_000u32 / fps);

        let mut previous_tick = Instant::now();
        let tick_duration = Duration::new(0, 1_000_000_000u32 / tps);

        'main: loop {
            while let Some(event) = event_pump.poll_event() {
                match event {
                    // handle exit
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => {
                        break 'main;
                    }

                    // handle keyboard input
                    Event::KeyDown { .. } | Event::KeyUp { .. } => self.handle_key(event),

                    // handle mouse input
                    Event::MouseButtonDown { .. }
                    | Event::MouseButtonUp { .. }
                    | Event::MouseMotion { .. }
                    | Event::MouseWheel { .. } => self.handle_mouse(event),

                    // ignore other events
                    _ => {}
                };
            }

            let timestamp = Instant::now();

            if timestamp >= previous_tick + tick_duration {
                self.tick();
                previous_tick = timestamp;
            }

            if timestamp >= previous_frame + frame_duration {
                self.render(&mut canvas);
                previous_frame = timestamp;
            }
        }

        Ok(())
    }
}
