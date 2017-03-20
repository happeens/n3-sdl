extern crate sdl2;
extern crate time;

use sdl2::image::INIT_PNG;

mod types;
mod game;
use game::Game;

mod sprite;
mod sprite_cache;
mod renderable;

mod player;
mod camera;
mod world;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();
    let _image_context = sdl2::image::init(INIT_PNG).unwrap();

    let window = video.window("n3", 800, 600)
        .position_centered()
        .opengl()
        .build().unwrap();

    let mut game = Game::new(
        sdl_context.event_pump().unwrap(),
        window.renderer().accelerated().build().unwrap());
    game.run();
}
