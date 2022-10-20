use boilerplate::GameEngine;
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

mod boilerplate;

fn main() -> Result<(), String> {
    println!("Hello, world!");

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

fn render_handler(canvas: &mut WindowCanvas) {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    let pix_w: u32 = 800 / 255 + 1;
    let pix_h: u32 = 600 / 255 + 1;

    for i in 0..256 {
        for j in 0..256 {
            canvas.set_draw_color(Color::RGB(i as u8, j as u8, 255));
            canvas
                .fill_rect(Rect::new(i * pix_w as i32, j * pix_h as i32, pix_w, pix_h))
                .unwrap();
        }
    }

    canvas.present();
}

fn tick_handler() {}

fn keyboard_handler(keyboard_event: Event) {
    match keyboard_event {
        Event::KeyDown { .. } => (),
        Event::KeyUp { .. } => (),
        _ => unreachable!(),
    }
}

fn mouse_handler(keyboard_event: Event) {
    match keyboard_event {
        Event::MouseButtonDown { .. } => (),
        Event::MouseButtonUp { .. } => (),
        Event::MouseMotion { .. } => (),
        Event::MouseWheel { .. } => (),
        _ => unreachable!(),
    }
}
