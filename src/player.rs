use sdl2::render::Renderer;
use renderable::Renderable;

use camera::Camera;
use sprite::Sprite;
use sprite::spritecache::SpriteCache;

use types::{Point, Size, KeyAction};

const PLAYER_SPEED: f64 = 130.0;

#[derive(PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

pub struct Player {
    pos: Point,
    vel: Point,
    facing: Direction,
    frames: Vec<Sprite>,
    current: usize
}

impl Player {
    pub fn new(start_pos: Point, sc: &SpriteCache) -> Player {
        let mut frames = Vec::new();
        for i in 0..4 {
            for j in 0..4 {
                let frame_name = format!("player-{}-{}", i, j);
                if let Some(frame) = sc.get_sprite(&frame_name) {
                    println!("found frame: {}", frame_name);
                    frames.push(frame);
                } else {
                    println!("missing player frame: {}", frame_name);
                }
            }
        }

        Player {
            pos: start_pos,
            vel: Point::new(0.0, 0.0),
            facing: Direction::Down,
            frames: frames,
            current: 1
        }
    }

    pub fn update(&mut self, dt: f64) {
        let vel = self.vel;
        self.pos = self.next_pos(dt, vel);
    }

    pub fn next_pos(&mut self, dt: f64, vel: Point) -> Point {
        self.pos + (vel * dt * PLAYER_SPEED)
    }

    pub fn get_pos(&self) -> Point {
        self.pos
    }

    pub fn get_vel(&self) -> Point {
        self.vel
    }

    pub fn set_vel(&mut self, vel: Point) {
        self.vel = vel;
    }

    pub fn set_facing(&mut self, dir: Direction) {
        if dir == self.facing { return; }
        match dir {
            Direction::Down => self.current = 1,
            Direction::Up => self.current = 5,
            Direction::Left => self.current = 9,
            Direction::Right => self.current = 13
        }

        self.facing = dir;
    }

    pub fn draw(&self, mut r: &mut Renderer, c: &Camera) {
        if self.current > self.frames.len() {
            println!("invalid frame for player: {}", self.current);
            self.frames[0].draw(self.pos, r, c);
            return;
        }
        self.frames[self.current].draw(self.pos, r, c);
    }
}
