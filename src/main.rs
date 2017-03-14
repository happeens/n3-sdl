extern crate sdl2;
extern crate time;

mod game;
use game::Game;

mod world;
mod player;
mod camera;
mod types;
mod renderable;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();

    let window = video.window("n3", 800, 600)
        .position_centered()
        .opengl()
        .build().unwrap();

    let mut game = Game::new(
        sdl_context.event_pump().unwrap(),
        window.renderer().accelerated().build().unwrap());
    game.run();
}
