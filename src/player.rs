use sdl2::pixels::Color;
use sdl2::render::Renderer;

use types::{Point, Size};
use camera::Camera;

const PLAYER_SPEED: f64 = 130.0;

pub struct Player {
    pos: Point,
    anchor: Point,
    size: Size,
    vel: Point,
    color: Color,
}

impl Player {
    pub fn new() -> Player {
        Player {
            pos: Point::new(0.0, 0.0),
            anchor: Point::new(20.0 / 2.0, 30.0 / 2.0),
            size: Size::new(20.0, 30.0),
            vel: Point::new(0.0, 0.0),
            color: Color::RGB(255, 255, 255),
        }
    }

    pub fn update(&mut self, dt: f64) {
        let vel = self.vel;
        self.pos = self.next_pos(dt, vel);
    }

    pub fn draw(&self, r: &mut Renderer, c: &Camera) {
        r.set_draw_color(self.color);
        let pos = self.pos - c.get_pos() - self.anchor;
        let _ = r.fill_rect(pos.to_sdl_rect(self.size));
    }

    pub fn next_pos(&mut self, dt: f64, vel: Point) -> Point {
        self.pos + (vel * dt * PLAYER_SPEED)
    }

    pub fn get_vel(&self) -> Point {
        self.vel
    }

    pub fn set_vel(&mut self, vel: Point) {
        self.vel = vel;
    }

    pub fn set_pos(&mut self, pos: Point) {
        self.pos = pos;
    }

    pub fn get_pos(&self) -> Point {
        self.pos
    }
}
