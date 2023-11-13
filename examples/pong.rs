extern crate rgbp;

use rgbp::GameEngine;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::WindowCanvas;

fn main() -> Result<(), String> {
    Pong::new().run()
}

struct Pong {
    ball: Point,
    ball_vel: Point,
    player: Point,
    player_vel: Point,
    deaths: u32,
}

impl Pong {
    pub fn new() -> Self {
        Self {
            player: Point::new(400, 550),
            player_vel: Point::new(0, 0),
            ball: Point::new(100, 200),
            ball_vel: Point::new(10, 10),
            deaths: 0,
        }
    }
}

impl GameEngine for Pong {
    fn config(&mut self) -> (&str, u32, u32, u32, u32) {
        const TITLE: &str = "Pong";
        const WIDTH: u32 = 800;
        const HEIGHT: u32 = 600;
        const FPS: u32 = 60;
        const TPS: u32 = 20;

        (TITLE, WIDTH, HEIGHT, FPS, TPS)
    }

    fn handle_key(&mut self, keyboard_event: Event) {
        let vel = 15;
        match keyboard_event {
            Event::KeyDown { keycode, .. } => match keycode {
                Some(Keycode::W | Keycode::Up) if self.player_vel.y >= 0 => {
                    self.player_vel.y -= vel
                }
                Some(Keycode::A | Keycode::Left) if self.player_vel.x >= 0 => {
                    self.player_vel.x -= vel
                }
                Some(Keycode::S | Keycode::Down) if self.player_vel.y <= 0 => {
                    self.player_vel.y += vel
                }
                Some(Keycode::D | Keycode::Right) if self.player_vel.x <= 0 => {
                    self.player_vel.x += vel
                }
                None | Some(_) => {}
            },
            Event::KeyUp { keycode, .. } => match keycode {
                Some(Keycode::W | Keycode::Up) if self.player_vel.y <= 0 => {
                    self.player_vel.y += vel
                }
                Some(Keycode::A | Keycode::Left) if self.player_vel.x <= 0 => {
                    self.player_vel.x += vel
                }
                Some(Keycode::S | Keycode::Down) if self.player_vel.y >= 0 => {
                    self.player_vel.y -= vel
                }
                Some(Keycode::D | Keycode::Right) if self.player_vel.x >= 0 => {
                    self.player_vel.x -= vel
                }
                None | Some(_) => {}
            },
            _ => unreachable!(),
        }
    }

    fn render(&mut self, canvas: &mut WindowCanvas) {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        // update player
        self.player.x = (self.player.x + self.player_vel.x).clamp(75, 800 - 75);
        let player_rect = Rect::new(self.player.x - 75, self.player.y - 10, 150, 20);
        canvas.set_draw_color(Color::RGB(0, 255, 0));
        canvas.fill_rect(player_rect).unwrap();

        // update ball
        let next_ball_pos = self.ball + self.ball_vel;
        let collides_with_player = player_rect.has_intersection(Rect::new(
            next_ball_pos.x - 20,
            next_ball_pos.y - 20,
            40,
            40,
        ));

        // horizontal collision
        if next_ball_pos.x < 20 || next_ball_pos.x > 800 - 20 {
            self.ball_vel.x *= -1;
            self.ball.x = next_ball_pos.x.clamp(20, 800 - 20);
        } else if collides_with_player {
            let dx = self.ball.x - self.player.x;
            self.ball_vel.x = dx.signum() * (dx.abs() / 3).clamp(3, 30);
        } else {
            self.ball.x = next_ball_pos.x;
        }

        // reset
        if next_ball_pos.y > 600 - 20 {
            self.ball = Point::new(400, 300);
            self.ball_vel.y *= -1;
            self.deaths += 1;
            println!("Death count: {}", self.deaths);
        // top collision
        } else if next_ball_pos.y < 20 {
            self.ball_vel.y *= -1;
            self.ball.y = next_ball_pos.y.clamp(20, 600 - 20);
        // player collision
        } else if collides_with_player {
            self.ball_vel.y = (self.ball.y - self.player.y) / 3;
            if self.ball_vel.y == 0 {
                self.ball_vel.y = -10;
            } else if self.ball_vel.y < 0 {
                self.ball.y -= 1;
            }
        } else {
            self.ball.y = next_ball_pos.y;
        }

        let ball_rect = Rect::new(self.ball.x - 20, self.ball.y - 20, 40, 40);
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.fill_rect(ball_rect).unwrap();

        canvas.present();
    }
}
