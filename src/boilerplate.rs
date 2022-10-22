use std::time::{Duration, Instant};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::WindowCanvas;
use sdl2::EventPump;

type TickHandler<GameState> = fn(Option<&mut GameState>);
type RenderHandler<GameState> = fn(&mut WindowCanvas, Option<&mut GameState>);
type KeyboardHandler<GameState> = fn(Event, Option<&mut GameState>);
type MouseHandler<GameState> = fn(Event, Option<&mut GameState>);

pub struct GameEngine<GameState> {
    fps: u32,
    tps: u32,
    canvas: WindowCanvas,
    event_pump: EventPump,
    state: Option<GameState>,

    // handlers
    tick_handler: Option<TickHandler<GameState>>,
    render_handler: Option<RenderHandler<GameState>>,
    keyboard_handler: Option<KeyboardHandler<GameState>>,
    mouse_handler: Option<MouseHandler<GameState>>,
}

impl<GameState> GameEngine<GameState> {
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window(title, width, height)
            .position_centered()
            .build()
            .expect("could not initialize the video subsystem");

        Self {
            fps: 60,
            tps: 20,
            canvas: window.into_canvas().build().unwrap(),
            event_pump: sdl_context.event_pump().unwrap(),
            state: None,
            tick_handler: None,
            render_handler: None,
            keyboard_handler: None,
            mouse_handler: None,
        }
    }

    pub fn set_fps(mut self, fps: u32) -> Self {
        self.fps = fps;
        self
    }

    pub fn set_tps(mut self, tps: u32) -> Self {
        self.tps = tps;
        self
    }

    pub fn set_state(mut self, state: GameState) -> Self {
        self.state = Some(state);
        self
    }

    pub fn set_tick_handler(mut self, tick_handler: TickHandler<GameState>) -> Self {
        self.tick_handler = Some(tick_handler);
        self
    }

    pub fn set_render_handler(mut self, render_handler: RenderHandler<GameState>) -> Self {
        self.render_handler = Some(render_handler);
        self
    }

    pub fn set_keyboard_handler(mut self, keyboard_handler: KeyboardHandler<GameState>) -> Self {
        self.keyboard_handler = Some(keyboard_handler);
        self
    }

    pub fn set_mouse_handler(mut self, mouse_handler: MouseHandler<GameState>) -> Self {
        self.mouse_handler = Some(mouse_handler);
        self
    }

    pub fn run(&mut self) -> Result<(), String> {
        if self.fps == 0 || self.tps == 0 {
            return Err("fps and tps cannot be 0".to_string());
        }

        let mut previous_frame = Instant::now();
        let frame_duration = Duration::new(0, 1_000_000_000u32 / self.fps);

        let mut previous_tick = Instant::now();
        let tick_duration = Duration::new(0, 1_000_000_000u32 / self.tps);

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
                        self.keyboard_handler
                            .map(|handle| handle(event, self.state.as_mut()));
                    }

                    // handle mouse input
                    Event::MouseButtonDown { .. }
                    | Event::MouseButtonUp { .. }
                    | Event::MouseMotion { .. }
                    | Event::MouseWheel { .. } => {
                        self.mouse_handler
                            .map(|handle| handle(event, self.state.as_mut()));
                    }

                    // ignore other events
                    _ => {}
                };
            }

            let timestamp = Instant::now();

            if timestamp >= previous_tick + tick_duration {
                self.tick_handler.map(|tick| tick(self.state.as_mut()));
                previous_tick = timestamp;
            }

            if timestamp >= previous_frame + frame_duration {
                self.render_handler
                    .map(|render| render(&mut self.canvas, self.state.as_mut()));
                previous_frame = timestamp;
            }
        }

        Ok(())
    }
}
