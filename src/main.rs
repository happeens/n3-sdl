extern crate sdl2;
extern crate cgmath;
extern crate time;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

extern crate base64;
extern crate byteorder;

use sdl2::image::INIT_PNG;

mod types;
mod context;
mod scene;

mod sprite;
mod animation;
mod state;

mod entities;
mod camera;
mod tilemap;

mod util;

fn main() {
    let mut main_ctx = context::Context::new();
    let mut game_scene = scene::GameScene::new(&mut main_ctx);
    main_ctx.run_scene(&mut game_scene);
}
