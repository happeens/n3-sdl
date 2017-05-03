use time::{Duration, PreciseTime};
use std::cmp;
use std::thread::sleep;

use sdl2::EventPump as SdlEvents;
use sdl2::render::Renderer as SdlRenderer;
use sdl2::render::Texture;
use sdl2::image::{INIT_PNG, LoadTexture};

use std::path::Path;

use std::cell::RefCell;
use std::rc::Rc;

use sdl2::event::Event::*;
use sdl2::keyboard::Keycode::*;

use scene::Scene;
use camera::Camera;
use sprite::SpriteCache;

use types::{KeyAction, Point, Size, Drawable, Color};
use types::to_sdl_rect;

const CAMERA_SPEED: f64 = 2.0;
const WINDOW_W: u32 = 1280;
const WINDOW_H: u32 = 800;

const NANOS_IN_SECOND: f64 = 1000000000.0;
const STEP_NS: f64 = NANOS_IN_SECOND / 60.0;

pub struct Context<'renderer> {
    running: bool,
    events: SdlEvents,
    renderer: SdlRenderer<'renderer>,
    sprite_cache: SpriteCache,
    camera: Camera,
    held_keys: Vec<KeyAction>,
}

impl<'renderer> Context<'renderer> {
    pub fn new() -> Context<'renderer> {
        let sdl_context = super::sdl2::init().unwrap();
        let video = sdl_context.video().unwrap();
        let _image_context = super::sdl2::image::init(INIT_PNG).unwrap();

        let window = video.window("n3-ctx", WINDOW_W, WINDOW_H)
            .position_centered().opengl()
            .build().unwrap();

        let mut sc = SpriteCache::new();
        let c = Camera::new(Point::new(0.0, 0.0),
                            Size::new(WINDOW_W as f64, WINDOW_H as f64),
                            CAMERA_SPEED);

        Context {
            running: false,
            events: sdl_context.event_pump().unwrap(),
            renderer: window.renderer().accelerated().build().unwrap(),
            sprite_cache: sc,
            camera: c,
            held_keys: Vec::new()
        }
    }

    pub fn run_scene<T>(&mut self, s: &mut T) where T: Scene {
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
                self.handle_events();
                s.update(self, dt);
                self.camera.update(dt);
            }

            self.renderer.set_draw_color(Color::RGB(0, 0, 0));
            self.renderer.clear();
            s.draw(self);
            self.renderer.present();

            // limit to 60 fps
            let render_time = current_time.to(PreciseTime::now());
            let difference = step - render_time;
            if difference > Duration::zero() {
                sleep(difference.to_std().unwrap());
            }
        }
    }

    pub fn load_sheet(&mut self, name: &str) {
        self.sprite_cache.load_sheet(name, &mut self.renderer);
    }

    pub fn load_texture(&mut self, name: &str) -> Rc<RefCell<Texture>> {
        let path = String::from("assets/") + name;
        let tex = self.renderer.load_texture(Path::new(&path)).unwrap();
        Rc::new(RefCell::new(tex))
    }

    pub fn get_sprite_cache(&self) -> &SpriteCache {
        &self.sprite_cache
    }

    pub fn held_keys(&self) -> super::std::slice::Iter<KeyAction> {
        self.held_keys.iter()
    }

    pub fn last_key(&self) -> Option<&KeyAction> {
        self.held_keys.last()
    }

    pub fn set_camera_target(&mut self, t: Point) {
        self.camera.set_target(t);
    }

    pub fn draw_texture<T>(&mut self, pos: Point, d: &T) where T: Drawable {
        let dest = pos + (self.camera.as_vec() * -1.0);
        let _ = self.renderer.copy(&mut d.get_tex(),
                                   Some(to_sdl_rect(d.get_src(), d.get_src_size())),
                                   Some(to_sdl_rect(dest, d.get_size())));
    }

    pub fn draw_rect(&mut self, pos: Point, s: Size, c: Color) {
        let dest = pos + (self.camera.as_vec() * -1.0);
        self.renderer.set_draw_color(c);
        let _ = self.renderer.fill_rect(Some(to_sdl_rect(dest, s)));
    }

    fn handle_events(&mut self) {
        for event in self.events.poll_iter() {
            match event {
                Quit { .. } => self.running = false,
                KeyDown { keycode, repeat, .. } => {
                    if repeat {
                        continue;
                    }

                    match keycode {
                        Some(Escape) => self.running = false,
                        Some(W) => self.held_keys.push(KeyAction::Up),
                        Some(A) => self.held_keys.push(KeyAction::Left),
                        Some(S) => self.held_keys.push(KeyAction::Down),
                        Some(D) => self.held_keys.push(KeyAction::Right),
                        _ => {}
                    }
                }
                KeyUp { keycode, .. } => {
                    match keycode {
                        Some(W) => self.held_keys.retain(|&x| x != KeyAction::Up),
                        Some(A) => self.held_keys.retain(|&x| x != KeyAction::Left),
                        Some(S) => self.held_keys.retain(|&x| x != KeyAction::Down),
                        Some(D) => self.held_keys.retain(|&x| x != KeyAction::Right),
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }
}