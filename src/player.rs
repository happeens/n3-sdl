use sdl2::render::Renderer;
use renderable::Renderable;

use camera::Camera;
use sprite::Sprite;
use sprite::spritecache::SpriteCache;

use types::{Point, Size};

const PLAYER_SPEED: f64 = 130.0;

pub struct Player {
    sprite: Sprite,
    vel: Point,
}

impl Player {
    pub fn new(start_pos: Point, sprite: Sprite) -> Player {
        Player {
            sprite: sprite,
            vel: Point::new(0.0, 0.0),
        }
    }

    pub fn update(&mut self, dt: f64) {
        let vel = self.vel;
        let next_pos = self.next_pos(dt, vel);
        self.sprite.set_pos(next_pos);
    }

    pub fn next_pos(&mut self, dt: f64, vel: Point) -> Point {
        self.sprite.get_pos() + (vel * dt * PLAYER_SPEED)
    }

    pub fn get_pos(&self) -> Point {
        self.sprite.get_pos()
    }

    pub fn get_vel(&self) -> Point {
        self.vel
    }

    pub fn set_vel(&mut self, vel: Point) {
        self.vel = vel;
    }

    pub fn draw(&self, mut r: &mut Renderer, c: &Camera) {
        self.sprite.draw(r, c);
    }
}
