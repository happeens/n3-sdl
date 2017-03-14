use time::{Duration, PreciseTime};
use std::collections::HashSet;
use std::cmp;

use sdl2::EventPump as SdlEvents;
use sdl2::render::Renderer;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

use sdl2::event::Event::*;
use sdl2::keyboard::Keycode::*;

use world::World;
use player::Player;
use camera::Camera;

use types::{TilePos, Point, Size, KeyAction};

const NANOS_IN_SECOND: f64 = 1000000000.0;
const STEP_NS: f64 = NANOS_IN_SECOND / 60.0;

const CAMERA_SPEED: f64 = 2.0;

pub struct Game<'a> {
    world: World,
    player: Player,
    camera: Camera,
    running: bool,
    events: SdlEvents,
    renderer: Renderer<'a>,
    held_keys: HashSet<KeyAction>,
}

impl<'a> Game<'a> {
    pub fn new(e: SdlEvents, r: Renderer) -> Game {
        let world = World::new();
        let start_pos = world.from_tile_pos(TilePos::new(3, 3));
        let mut player = Player::new(start_pos);
        let camera = Camera::new(
            player.get_r().pos,
            Size::new(800.0, 600.0),
            CAMERA_SPEED);

        Game {
            world: world,
            player: player,
            camera: camera,
            running: false,
            events: e,
            renderer: r,
            held_keys: HashSet::new(),
        }
    }

    pub fn run(&mut self) {
        self.running = true;
        let mut current_time = PreciseTime::now();
        let step = Duration::nanoseconds(STEP_NS.floor() as i64);

        while self.running {
            let new_time = PreciseTime::now();
            let mut frame_time = current_time.to(new_time);
            current_time = new_time;

            while frame_time > Duration::zero() {
                let dt = cmp::min(frame_time, step);
                frame_time = frame_time - dt;

                // convert to seconds and update game state
                let dt = dt.num_nanoseconds().unwrap() as f64 / NANOS_IN_SECOND;
                self.update(dt);
            }

            self.draw();
        }
    }

    pub fn update(&mut self, dt: f64) {
        for event in self.events.poll_iter() {
            match event {
                Quit { .. } => self.running = false,
                KeyDown { keycode, .. } => match keycode {
                    Some(Escape) => self.running = false,
                    Some(W) => { self.held_keys.insert(KeyAction::Up); },
                    Some(A) => { self.held_keys.insert(KeyAction::Left); },
                    Some(S) => { self.held_keys.insert(KeyAction::Down); },
                    Some(D) => { self.held_keys.insert(KeyAction::Right); },
                    _ => {},
                },
                KeyUp { keycode, .. } => match keycode {
                    Some(W) => { self.held_keys.remove(&KeyAction::Up); },
                    Some(A) => { self.held_keys.remove(&KeyAction::Left); },
                    Some(S) => { self.held_keys.remove(&KeyAction::Down); },
                    Some(D) => { self.held_keys.remove(&KeyAction::Right); },
                    _ => {}
                },
                _ => {}
            }
        }

        let mut move_intention = Point::new(0.0, 0.0);

        for key in self.held_keys.iter() {
            match key {
                &KeyAction::Up => move_intention.add_y(-1.0),
                &KeyAction::Down => move_intention.add_y(1.0),
                &KeyAction::Left => move_intention.add_x(-1.0),
                &KeyAction::Right => move_intention.add_x(1.0),
            }
        }

        if move_intention.is_diag() {
            move_intention.mult_diag();
        }

        self.try_move_player(move_intention, dt);

        self.world.update(dt);

        let player_pos = self.player.get_r().pos;
        let player_tile_pos = self.world.to_tile_pos(player_pos);
        self.world.set_highlighted(player_tile_pos);

        self.camera.set_target(player_pos);
        self.camera.update(dt);

        self.player.update(dt);
    }

    pub fn draw(&mut self) {
        self.renderer.set_draw_color(Color::RGB(0, 0, 0));
        self.renderer.clear();

        self.world.draw(&mut self.renderer, &self.camera);
        self.player.get_r().draw(&mut self.renderer, &self.camera);

        self.renderer.present();
    }

    pub fn try_move_player(&mut self, v: Point, dt: f64) {
        let mut v = v;

        if self.world.check_pos_collides(self.player.next_pos(dt, v)) {
            let new_x = self.player.next_pos(dt, Point::new(v.x(), 0.0));
            let new_y = self.player.next_pos(dt, Point::new(0.0, v.y()));

            if self.world.check_pos_collides(new_x) {
                v.set_x(0.0);
            }

            if self.world.check_pos_collides(new_y) {
                v.set_y(0.0);
            }

            v.mult_diag();
            self.player.set_vel(v);
        }

        self.player.set_vel(v);
    }
}
