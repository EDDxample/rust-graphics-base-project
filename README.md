# Rust Graphics Base Project

Rust abstraction over SDL 2.0 to write 🔥blazing fast🔥 sketches with minimal overhead.

## Examples (check the examples folder)
- [x] Pong
- [ ] Snake
- [ ] Tetris
- [ ] Mandelbrot
- [ ] Rotating 4D cube and other polytopes

## Getting Started
- Clone this repo
- Download the [SDL2 development library](https://github.com/libsdl-org/SDL/releases) and follow [this documentation](https://crates.io/crates/sdl2) to put the binary and header files in the right locations
- Run the examples
```
cargo run --example pong
```

## Sketch Layout
```rs
fn main() -> Result<(), String> {
    MyGame::new().run()
}

struct MyGame { /* define game state */ }

impl MyGame {
    pub fn new() -> Self {
        Self { /* init game state */ }
    }
}

impl GameEngine for MyGame {
    fn config(&mut self) -> (&str, u32, u32, u32, u32) {
        /// basic window and game config
        const TITLE: &str = "My Game";
        const WIDTH: u32 = 500;
        const HEIGHT: u32 = 500;
        const FPS: u32 = 60;  // render speed
        const TPS: u32 = 20;  // logic speed

        (TITLE, WIDTH, HEIGHT, FPS, TPS)
    }

    fn tick(&mut self) {
        // update game logic
    }

    fn render(&mut self, canvas: &mut WindowCanvas) {
        // update game display
    }

    fn handle_key(&mut self, event: Event) {
        // handle Event::{KeyDown, KeyUp}
    }

    fn handle_mouse(&mut self, event: Event) {
        // handle Event::{MouseButtonDown, MouseButtonUp, MouseMotion, MouseWheel}
    }
}
```
