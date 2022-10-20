# Rust Graphics Base Project

Rust abstraction over SDL 2.0 to write 🔥blazing fast🔥 sketches with minimal overhead.

## Features
- [x] Basic interface (tick, render and input event handlers)
- [x] Example program 
- [ ] Screenshot support
- [ ] WGPU shader support
- [ ] ...
- [ ] Profit

## Examples (check branches)
- [ ] Pong
- [ ] Snake
- [ ] Tetris
- [ ] Mandelbrot
- [ ] Rotating 4D cube and other polytopes

## Getting Started
- Clone this repo
- Download the [SDL2 development library](https://github.com/libsdl-org/SDL/releases) and follow [this documentation](https://crates.io/crates/sdl2) to put the binary and header files in the right locations
- Run this command
```
cargo run
```

## Sketch Layout
```rs
fn main() -> Result<(), String> {
    let mut game = GameEngine::new("hello world", 800, 600)
        .set_tps(20)
        .set_fps(60)
        .set_tick_handler(tick_handler)
        .set_render_handler(render_handler)
        .set_keyboard_handler(keyboard_handler)
        .set_mouse_handler(mouse_handler);
    game.run()?;

    Ok(())
}

fn tick_handler() {
    // update game logic
}

fn render_handler(canvas: &mut WindowCanvas) {
    // update game display
}

fn keyboard_handler(event: Event) {
    // handle Event::{KeyDown, KeyUp}
}

fn mouse_handler(event: Event) {
    // handle Event::{MouseButtonDown, MouseButtonUp, MouseMotion, MouseWheel}
}
```