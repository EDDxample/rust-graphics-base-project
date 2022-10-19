use std::time::{Duration, Instant};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::WindowCanvas;
use sdl2::EventPump;

type RenderHandler = fn(&mut WindowCanvas);
type KeyboardHandler = fn(Event);
type MouseHandler = fn(Event);

struct Engine {
    fps: u32,
    canvas: WindowCanvas,
    event_pump: EventPump,

    // handlers
    render_handler: Option<RenderHandler>,
    keyboard_handler: Option<KeyboardHandler>,
    mouse_handler: Option<MouseHandler>,
}

pub struct Game {
    engine: Engine,
}

impl Game {
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        Self {
            engine: Engine::new(title, width, height, 60),
        }
    }

    pub fn set_fps(mut self, fps: u32) -> Self {
        self.engine.fps = fps;
        self
    }

    pub fn run(&mut self) -> Result<(), String> {
        self.engine.gameloop()
    }

    pub fn set_render_handler(mut self, render_handler: RenderHandler) -> Self {
        self.engine.render_handler = Some(render_handler);
        self
    }

    pub fn set_keyboard_handler(mut self, keyboard_handler: KeyboardHandler) -> Self {
        self.engine.keyboard_handler = Some(keyboard_handler);
        self
    }

    pub fn set_mouse_handler(mut self, mouse_handler: MouseHandler) -> Self {
        self.engine.mouse_handler = Some(mouse_handler);
        self
    }
}

impl Engine {
    fn new(title: &str, width: u32, height: u32, fps: u32) -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window(title, width, height)
            .position_centered()
            .build()
            .expect("could not initialize the video subsystem");

        Self {
            fps,
            canvas: window.into_canvas().build().unwrap(),
            event_pump: sdl_context.event_pump().unwrap(),
            render_handler: None,
            keyboard_handler: None,
            mouse_handler: None,
        }
    }
    fn gameloop(&mut self) -> Result<(), String> {
        let mut previous_frame = Instant::now();
        let frame_duration = Duration::new(0, 1_000_000_000u32 / self.fps);

        'main: loop {
            for event in self.event_pump.poll_event() {
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
                    Event::KeyDown { .. } | Event::KeyUp { .. } => {
                        self.keyboard_handler.map(|handle| handle(event));
                    }

                    // handle mouse input
                    Event::MouseButtonDown { .. }
                    | Event::MouseButtonUp { .. }
                    | Event::MouseMotion { .. }
                    | Event::MouseWheel { .. } => {
                        self.mouse_handler.map(|handle| handle(event));
                    }

                    // ignore the rest of events
                    _ => {}
                };
            }

            // call the render handler on time for the next frame
            let timestamp = Instant::now();
            if timestamp >= previous_frame + frame_duration {
                self.render_handler.map(|render| render(&mut self.canvas));
                previous_frame = timestamp;
            }
        }

        Ok(())
    }
}
