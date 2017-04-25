use sdl2::render::Renderer;

use std::fs::File;
use std::path::Path;
use std::io::prelude::*;

use camera::Camera;
use sprite::Sprite;
use sprite::spritecache::SpriteCache;

use animation::Animation;

use types::KeyAction;

use cgmath::{Point2, Vector2};

const PLAYER_SPEED: f64 = 130.0;

#[derive(PartialEq, Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

pub struct Player {
    pos: Point2<f64>,
    vel: Vector2<f64>,
    facing: Direction,
    frames: Vec<Sprite>,
    current: usize,

    //TODO move animations into seperate struct
    anims: Vec<Animation>,
    anim_timer: f64,
    anim_running: bool,
    current_anim: usize,
    current_frame: usize
}

impl Player {
    pub fn new(start_pos: Point2<f64>, sc: &SpriteCache) -> Player {
        let mut frames = Vec::new();
        for i in 0..4 {
            for j in 0..4 {
                let frame_name = format!("player-{}-{}", i, j);
                if let Some(frame) = sc.get_sprite(&frame_name) {
                    frames.push(frame);
                } else {
                    println!("missing player frame: {}", frame_name);
                }
            }
        }

        //TODO centralize metadata loading
        let path = String::from("assets/testanim.json");
        let mut file = File::open(Path::new(&path)).unwrap();
        let mut content = String::new();
        let _ = file.read_to_string(&mut content);

        let anims = Animation::load(&content, &frames);

        Player {
            pos: start_pos,
            vel: Vector2::new(0.0, 0.0),
            facing: Direction::Down,
            frames: frames,
            current: 1,
            anims: anims,
            anim_timer: 0.0,
            anim_running: false,
            current_anim: 0,
            current_frame: 0
        }
    }

    pub fn update(&mut self, dt: f64) {
        let vel = self.vel;
        self.pos = self.next_pos(dt, vel);

        if self.anim_running {
            self.advance_anim(dt);
            if !self.is_moving() {
                self.anim_running = false;
            }
        }
    }

    fn advance_anim(&mut self, dt: f64) {
        self.anim_timer += dt;

        //TODO rewrite this to look nicer
        if self.anim_timer > self.anims[self.current_anim]
            .frames[self.current_frame].delay {
            self.anim_timer = 0.0;
            self.current_frame += 1;
            if self.current_frame >= self.anims[self.current_anim].frames.len() {
                self.current_frame = 0;
            }
        }
    }

    pub fn run_anim(&mut self, name: &str) {
        if self.anim_running && self.anims[self.current_anim].name == name {
            return;
        }

        self.anim_running = true;
        self.anim_timer = 0.0;
        self.current_frame = 0;
        self.current_anim = 0;

        for (i, anim) in self.anims.iter().enumerate() {
            if anim.name == name {
                self.current_anim = i;
            }
        }
    }

    pub fn is_moving(&self) -> bool {
        self.vel.y != 0.0 || self.vel.x != 0.0
    }

    pub fn next_pos(&mut self, dt: f64, vel: Vector2<f64>) -> Point2<f64> {
        self.pos + (vel * dt * PLAYER_SPEED)
    }

    pub fn get_pos(&self) -> Point2<f64> {
        self.pos
    }

    pub fn get_vel(&self) -> Vector2<f64> {
        self.vel
    }

    pub fn set_vel(&mut self, vel: Vector2<f64>) {
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

    pub fn get_facing(&self) -> Direction {
        self.facing
    }

    pub fn draw(&self, mut r: &mut Renderer, c: &Camera) {
        if self.current > self.frames.len() {
            println!("invalid frame for player: {}", self.current);
            self.frames[0].draw(self.pos, r, c);
            return;
        }

        if self.anim_running {
            self.frames[self.anims[self.current_anim].frames[self.current_frame].index].draw(self.pos, r, c);
        } else {
            self.frames[self.current].draw(self.pos, r, c);
        }
    }
}
