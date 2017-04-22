use time::{Duration, PreciseTime};
use std::cmp;
use std::thread::sleep;

use sdl2::EventPump as SdlEvents;
use sdl2::render::Renderer;
use sdl2::pixels::Color;

use sdl2::event::Event::*;
use sdl2::keyboard::Keycode::*;

use sprite::spritecache::SpriteCache;
use world::World;
use player::{Player, Direction};
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
    sprite_cache: SpriteCache,
    held_keys: Vec<KeyAction>,
}

impl<'a> Game<'a> {
    pub fn new(e: SdlEvents, mut r: Renderer) -> Game {
        let mut sc = SpriteCache::new();
        sc.load_sheet("test", &mut r);

        let world = World::new(&mut r);
        let start_pos = Point::new(0.0, 0.0);

        let mut player = Player::new(start_pos, &sc);
        let camera = Camera::new(
            player.get_pos(),
            Size::new(800.0, 600.0),
            CAMERA_SPEED);


        Game {
            world: world,
            player: player,
            camera: camera,
            running: false,
            events: e,
            renderer: r,
            sprite_cache: sc,
            held_keys: Vec::new(),
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

            // do an update for every frame we rendered
            while frame_time > Duration::zero() {
                let dt = cmp::min(frame_time, step);
                frame_time = frame_time - dt;

                // convert to seconds and update game state
                let dt = dt.num_nanoseconds().unwrap() as f64 / NANOS_IN_SECOND;
                self.update(dt);
            }

            self.draw();

            // limit to 60 fps
            let render_time = current_time.to(PreciseTime::now());
            let difference = step - render_time;
            if difference > Duration::zero() {
                sleep(difference.to_std().unwrap());
            }
        }
    }

    pub fn update(&mut self, dt: f64) {
        for event in self.events.poll_iter() {
            match event {
                Quit { .. } => self.running = false,
                KeyDown { keycode, repeat, .. } => {
                    if repeat { continue; }

                    match keycode {
                        Some(Escape) => self.running = false,
                        Some(W) => self.held_keys.push(KeyAction::Up),
                        Some(A) => self.held_keys.push(KeyAction::Left),
                        Some(S) => self.held_keys.push(KeyAction::Down),
                        Some(D) => self.held_keys.push(KeyAction::Right),
                        _ => {},
                    }
                },
                KeyUp { keycode, .. } => match keycode {
                    Some(W) => self.held_keys.retain(|&x| x != KeyAction::Up),
                    Some(A) => self.held_keys.retain(|&x| x != KeyAction::Left),
                    Some(S) => self.held_keys.retain(|&x| x != KeyAction::Down),
                    Some(D) => self.held_keys.retain(|&x| x != KeyAction::Right),
                    _ => {}
                },
                _ => {}
            }
        }

        let mut move_intention = Point::new(0.0, 0.0);

        for key in self.held_keys.iter() {
            match key {
                &KeyAction::Up => move_intention.y -= 1.0,
                &KeyAction::Down => move_intention.y += 1.0,
                &KeyAction::Left => move_intention.x -= 1.0,
                &KeyAction::Right => move_intention.x += 1.0,
            }
        }

        match self.held_keys.last() {
            Some(&KeyAction::Up) => self.player.set_facing(Direction::Up),
            Some(&KeyAction::Down) => self.player.set_facing(Direction::Down),
            Some(&KeyAction::Left) => self.player.set_facing(Direction::Left),
            Some(&KeyAction::Right) => self.player.set_facing(Direction::Right),
            _ => {}
        }

        if move_intention.is_diag() {
            move_intention.mult_diag();
        }

        self.try_move_player(move_intention, dt);

        self.world.update(dt);

        let player_pos = self.player.get_pos();

        self.camera.set_target(player_pos);
        self.camera.update(dt);

        self.player.update(dt);
    }

    pub fn draw(&mut self) {
        self.renderer.set_draw_color(Color::RGB(0, 0, 0));
        self.renderer.clear();

        self.world.draw(&mut self.renderer, &self.camera);
        self.player.draw(&mut self.renderer, &self.camera);

        self.renderer.present();
    }

    pub fn try_move_player(&mut self, v: Point, dt: f64) {
        let mut v = v;

        // finding next player position for collision
        // let next_pos = self.player.next_pos(dt, v);
        // let new_x = self.player.next_pos(dt, Point::new(v.x(), 0.0));
        // let new_y = self.player.next_pos(dt, Point::new(0.0, v.y()));
        v.mult_diag();
        self.player.set_vel(v);
    }
}
