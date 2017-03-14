use sdl2::pixels::Color;
use renderable::Renderable;

use types::{Point, Size};

const PLAYER_SPEED: f64 = 130.0;

pub struct Player {
    r: Renderable,
    vel: Point,
}

impl Player {
    pub fn new(start_pos: Point) -> Player {
        let size = Size::new(20.0, 30.0);
        let color = Color::RGB(255, 255, 255);

        Player {
            r: Renderable::new(start_pos, size, color),
            vel: Point::new(0.0, 0.0),
        }
    }

    pub fn update(&mut self, dt: f64) {
        let vel = self.vel;
        self.r.pos = self.next_pos(dt, vel);
    }

    pub fn next_pos(&mut self, dt: f64, vel: Point) -> Point {
        self.r.pos + (vel * dt * PLAYER_SPEED)
    }

    pub fn get_vel(&self) -> Point {
        self.vel
    }

    pub fn set_vel(&mut self, vel: Point) {
        self.vel = vel;
    }

    pub fn get_r(&self) -> Renderable {
        self.r
    }
}
