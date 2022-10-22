#![allow(dead_code)]

use boilerplate::GameEngine;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::WindowCanvas;

mod boilerplate;

struct CustomState {
    ball: Point,
    ball_vel: Point,
    player: Point,
    player_vel: Point,
    deaths: u32,
}

fn main() -> Result<(), String> {
    println!("Hello, world!");

    let mut game = GameEngine::<CustomState>::new("basic pong", 800, 600)
        // .set_tps(20)
        // .set_tick_handler(tick_handler)
        .set_fps(60)
        .set_render_handler(render_handler)
        .set_keyboard_handler(keyboard_handler)
        .set_mouse_handler(mouse_handler)
        .set_state(CustomState {
            player: Point::new(400, 550),
            player_vel: Point::new(0, 0),
            ball: Point::new(100, 200),
            ball_vel: Point::new(10, 10),
            deaths: 0,
        });

    game.run()?;

    Ok(())
}

// fn tick_handler(state: Option<&mut CustomState>) {}

fn render_handler(canvas: &mut WindowCanvas, state: Option<&mut CustomState>) {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    state.map(|state| {
        // update player
        state.player.x = (state.player.x + state.player_vel.x).clamp(75, 800 - 75);
        let player_rect = Rect::new(state.player.x - 75, state.player.y - 10, 150, 20);
        canvas.set_draw_color(Color::RGB(0, 255, 0));
        canvas.fill_rect(player_rect).unwrap();

        // update ball
        let next_ball_pos = state.ball + state.ball_vel;
        let collides_with_player = player_rect.has_intersection(Rect::new(
            next_ball_pos.x - 20,
            next_ball_pos.y - 20,
            40,
            40,
        ));

        // horizontal collision
        if next_ball_pos.x < 20 || next_ball_pos.x > 800 - 20 {
            state.ball_vel.x *= -1;
            state.ball.x = next_ball_pos.x.clamp(20, 800 - 20);
        } else if collides_with_player {
            let dx = state.ball.x - state.player.x;
            state.ball_vel.x = dx.signum() * (dx.abs() / 3).clamp(3, 30);
        } else {
            state.ball.x = next_ball_pos.x;
        }

        // reset
        if next_ball_pos.y > 600 - 20 {
            state.ball = Point::new(400, 300);
            state.ball_vel.y *= -1;
            state.deaths += 1;
            println!("Death count: {}", state.deaths);
        // top collision
        } else if next_ball_pos.y < 20 {
            state.ball_vel.y *= -1;
            state.ball.y = next_ball_pos.y.clamp(20, 600 - 20);
        // player collision
        } else if collides_with_player {
            state.ball_vel.y = (state.ball.y - state.player.y) / 3;
            if state.ball_vel.y == 0 {
                state.ball_vel.y = -10;
            } else if state.ball_vel.y < 0 {
                state.ball.y -= 1;
            }
        } else {
            state.ball.y = next_ball_pos.y;
        }

        let ball_rect = Rect::new(state.ball.x - 20, state.ball.y - 20, 40, 40);
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.fill_rect(ball_rect).unwrap();
    });

    canvas.present();
}

fn keyboard_handler(keyboard_event: Event, state: Option<&mut CustomState>) {
    let vel = 15;
    state.map(|state| match keyboard_event {
        Event::KeyDown { keycode, .. } => match keycode {
            Some(Keycode::W | Keycode::Up) if state.player_vel.y >= 0 => state.player_vel.y -= vel,
            Some(Keycode::A | Keycode::Left) if state.player_vel.x >= 0 => {
                state.player_vel.x -= vel
            }
            Some(Keycode::S | Keycode::Down) if state.player_vel.y <= 0 => {
                state.player_vel.y += vel
            }
            Some(Keycode::D | Keycode::Right) if state.player_vel.x <= 0 => {
                state.player_vel.x += vel
            }
            None | Some(_) => {}
        },
        Event::KeyUp { keycode, .. } => match keycode {
            Some(Keycode::W | Keycode::Up) if state.player_vel.y <= 0 => state.player_vel.y += vel,
            Some(Keycode::A | Keycode::Left) if state.player_vel.x <= 0 => {
                state.player_vel.x += vel
            }
            Some(Keycode::S | Keycode::Down) if state.player_vel.y >= 0 => {
                state.player_vel.y -= vel
            }
            Some(Keycode::D | Keycode::Right) if state.player_vel.x >= 0 => {
                state.player_vel.x -= vel
            }
            None | Some(_) => {}
        },
        _ => unreachable!(),
    });
}

fn mouse_handler(mouse_event: Event, _: Option<&mut CustomState>) {
    match mouse_event {
        Event::MouseButtonDown { .. } => (),
        Event::MouseButtonUp { .. } => (),
        Event::MouseMotion { .. } => (),
        Event::MouseWheel { .. } => (),
        _ => unreachable!(),
    }
}
