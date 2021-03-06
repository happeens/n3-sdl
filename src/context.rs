use std::path::Path;
use std::cell::RefCell;
use std::rc::Rc;

use sdl2::EventPump as SdlEvents;
use sdl2::render::Renderer as SdlRenderer;
use sdl2::image::{INIT_PNG, LoadTexture};
use sdl2::event::Event::*;
use sdl2::keyboard::Keycode::*;

use time::{Duration, PreciseTime};

use scene::Scene;
use camera::Camera;
use sprite::SpriteCache;
use types::{KeyAction, Point, Size, Vec2, RenderInfo, Renderable, Color, Texture, to_sdl_rect};

const CAMERA_SPEED: f32 = 2.0;
const WINDOW_W: u32 = 1920;
const WINDOW_H: u32 = 1200;

const NANOS_IN_SECOND: f32 = 1000000000.0;
const STEP_NS: f32 = NANOS_IN_SECOND / 60.0;

pub struct Context<'renderer> {
    running: bool,
    events: SdlEvents,
    renderer: SdlRenderer<'renderer>,
    sprite_cache: SpriteCache,
    camera: Camera,
    held_keys: Vec<KeyAction>,
    render_buffer: Vec<RenderInfo>
}

impl<'renderer> Context<'renderer> {
    pub fn new() -> Context<'renderer> {
        let sdl_context = super::sdl2::init().unwrap();
        let video = sdl_context.video().unwrap();
        let _image_context = super::sdl2::image::init(INIT_PNG).unwrap();

        let window = video.window("n3-ctx", WINDOW_W, WINDOW_H)
            .fullscreen_desktop()
            .build().unwrap();

        let sc = SpriteCache::new();
        let c = Camera::new(Point::new(0.0, 0.0),
                            Size::new(WINDOW_W as f32, WINDOW_H as f32),
                            CAMERA_SPEED);

        Context {
            running: false,
            events: sdl_context.event_pump().unwrap(),
            renderer: window.renderer().accelerated().build().unwrap(),
            sprite_cache: sc,
            camera: c,
            held_keys: Vec::new(),
            render_buffer: Vec::new()
        }
    }

    pub fn run_scene<T>(&mut self, s: &mut T) where T: Scene {
        self.running = true;
        let mut current_time = PreciseTime::now();
        let step = Duration::nanoseconds(STEP_NS.floor() as i64);
        let dt = step.num_nanoseconds().unwrap() as f32 / NANOS_IN_SECOND;
        let max_frame_time = Duration::seconds(1);
        let mut accumulator = Duration::zero();

        while self.running {
            self.handle_events();

            let new_time = PreciseTime::now();
            let mut frame_time = current_time.to(new_time);
            current_time = new_time;

            if frame_time > max_frame_time {
                frame_time = max_frame_time;
            }
            
            accumulator = accumulator + frame_time;

            while accumulator >= step {
                accumulator = accumulator - step;

                s.update(self, dt);
                self.camera.update(dt);
            }

            self.renderer.set_draw_color(Color::RGB(0, 0, 0));
            self.renderer.clear();

            let a = (current_time.to(PreciseTime::now()).num_nanoseconds().unwrap() as f32 / NANOS_IN_SECOND) / dt;

            s.draw(self, a);

            self.present(a);
            self.renderer.present();
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

    pub fn render(&mut self, r: RenderInfo) {
        self.render_buffer.push(r);
    }

    fn present(&mut self, a: f32) {
        self.render_buffer.sort_by_key(|e| e.z as i32);;
        let camera_offset = self.camera.next_vec(a);

        for r in &self.render_buffer {
            use std::ops::DerefMut;
            match r.renderable {
                Renderable::Texture { src, src_size, ref tex } => {
                    copy_texture(&mut self.renderer,
                                 camera_offset,
                                 r.pos, r.size,
                                 src, src_size,
                                 tex.borrow_mut().deref_mut());
                },
                Renderable::Rect { color } => render_rect(&mut self.renderer,
                                                          camera_offset,
                                                          r.pos, r.size, color),
            }
        }

        self.render_buffer.clear();
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
                        // Some(R) => self.
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

fn copy_texture(r: &mut SdlRenderer, camera_offset: Vec2, pos: Point, size: Size, src: Point, src_size: Size, tex: &mut Texture) {
    let dest = pos + camera_offset;
    let _ = r.copy(tex,
                 Some(to_sdl_rect(src, src_size)),
                 Some(to_sdl_rect(dest, size)));
}

fn render_rect(r: &mut SdlRenderer, camera_offset: Vec2, pos: Point, size: Size, color: Color) {
    let dest = pos + camera_offset;
    r.set_draw_color(color);
    let _ =  r.fill_rect(Some(to_sdl_rect(dest, size)));
    r.set_draw_color(Color::RGB(0, 0, 0));
    let _ =  r.draw_rect(to_sdl_rect(dest, size));
}
