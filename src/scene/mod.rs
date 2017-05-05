mod game_scene;
pub type GameScene = game_scene::GameScene;

use context::Context;

pub trait Scene {
    fn update(&mut self, ctx: &mut Context, dt: f32);
    fn draw(&self, ctx: &mut Context, a: f32);
}