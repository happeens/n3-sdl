use sprite::{Sprite, SpriteData, SpriteCache};
use types::{Point, Size, Color, RenderInfo};
use sdl2::render::Renderer;
use camera::Camera;
use context::Context;

pub struct SpriteManager {
    sprites: Vec<Sprite>
}

impl SpriteManager {
    pub fn new(data: &Vec<SpriteData>, sc: &SpriteCache) -> SpriteManager {
        let mut sprites = Vec::new();

        for sprite_data in data {
            if let Some(sprite) = sc.get_sprite(&sprite_data.name) {
                sprites.push(sprite);
            } else {
                println!("missing player frame: {}", sprite_data.name);
            }
        }

        SpriteManager {
            sprites: sprites
        }
    }

    pub fn find_index(&self, name: &str) -> Option<usize> {
        for (i, sprite) in self.sprites.iter().enumerate() {
            if sprite.name == name {
                return Some(i);
            }
        }

        None
    }

    pub fn draw(&self, index: usize, pos: Point, ctx: &mut Context) {
        if index > self.sprites.len() {
            println!("invalid frame for player: {}", index);

            ctx.render(&RenderInfo::Rect {
                pos: pos,
                size: Size::new(20.0, 20.0),
                color: Color::RGB(255, 0, 0)
            });
            return;
        }

        ctx.render(&self.sprites[index].get_render_info(pos));
    }
}