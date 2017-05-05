use types::{Point, Vec2, Direction};
use context::Context;

use animation::{AnimManager, AnimData};
use sprite::{SpriteData, SpriteManager, SpriteCache};
use state::{StateData, StateManager};

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerData {
    speed: f32,
    start_state: String,
    frames: Vec<SpriteData>,
    states: Vec<StateData>,
    animations: Vec<AnimData>,
}

pub struct Player {
    pos: Point,
    vel: Vec2,
    speed: f32,
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

    pub fn next_pos(&self, dt: f32) -> Point {
        self.pos + (self.vel * dt * self.speed)
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

    pub fn update(&mut self, dt: f32) {
        self.pos = self.next_pos(dt);

        if self.anims.anim_running() && !self.is_moving() {
            self.anims.stop_anim();
        }

        if self.is_moving() {
            match self.facing {
                Direction::Up => self.run_anim("walk-up"),
                Direction::Down => self.run_anim("walk-down"),
                Direction::Left => self.run_anim("walk-left"),
                Direction::Right => self.run_anim("walk-right")
            }
        }

        self.anims.update(dt);
    }

    pub fn draw(&self, mut ctx: &mut Context, a: f32) {
        let mut index = self.states.current();
        if self.anims.anim_running() {
            index = self.anims.current();
        }

        let dest = self.next_pos(a);
        self.sprites.draw(index, dest, ctx);
    }
}
