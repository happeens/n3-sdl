use animation::{AnimManager, AnimData};
use state::{StateData, StateManager};
use sprite::{Sprite, SpriteData, SpriteManager, SpriteCache};
use types::{Point, Vec2, Direction};

use camera::Camera;
use sdl2::render::Renderer;

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerData {
    speed: f64,
    start_state: String,
    frames: Vec<SpriteData>,
    states: Vec<StateData>,
    animations: Vec<AnimData>,
}

pub struct Player {
    pos: Point,
    vel: Vec2,
    speed: f64,
    facing: Direction,
    sprites: SpriteManager,
    states: StateManager,
    anims: AnimManager,
}

impl Player {
    pub fn new(data: &PlayerData, start_pos: Point, sc: &SpriteCache) -> Player {
        let sprites = SpriteManager::new(&data.frames, sc);
        let mut states = StateManager::new(&data.states, &sprites);
        let anims = AnimManager::new(&data.animations, &sprites);

        states.set(&data.start_state);

        Player {
            pos: start_pos,
            vel: Vec2::new(0.0, 0.0),
            speed: data.speed,
            facing: Direction::Down,
            sprites: sprites,
            states: states,
            anims: anims
        }
    }

    pub fn update(&mut self, dt: f64) {
        let vel = self.vel;
        self.pos = self.next_pos(dt, vel);

        if self.anims.anim_running() && !self.is_moving() {
            self.anims.stop_anim();
        }

        if self.is_moving() {
            match self.facing {
                Direction::Up => self.run_anim("walk-up"),
                Direction::Down => self.run_anim("walk-down"),
                Direction::Left => self.run_anim("walk-left"),
                Direction::Right => self.run_anim("walk-right"),
                _ => {}
            }
        }

        self.anims.update(dt);
    }

    pub fn draw(&self, mut r: &mut Renderer, c: &Camera) {
        let mut index = self.states.current();
        if self.anims.anim_running() {
            index = self.anims.current();
        }

        self.sprites.draw(index, self.pos, r, c);
    }

    pub fn set_facing(&mut self, dir: Direction) {
        self.facing = dir;

        match self.facing {
            Direction::Down => self.states.set("idle-down"),
            Direction::Up => self.states.set("idle-up"),
            Direction::Left => self.states.set("idle-left"),
            Direction::Right => self.states.set("idle-right"),
        }
    }

    pub fn run_anim(&mut self, anim: &str) {
        self.anims.run(anim);
    }

    pub fn next_pos(&mut self, dt: f64, vel: Vec2) -> Point {
        self.pos + (vel * dt * self.speed)
    }

    pub fn get_pos(&self) -> Point {
        self.pos
    }

    pub fn set_vel(&mut self, vel: Vec2) {
        self.vel = vel;
    }

    pub fn is_moving(&self) -> bool {
        self.vel.y != 0.0 || self.vel.x != 0.0
    }
}