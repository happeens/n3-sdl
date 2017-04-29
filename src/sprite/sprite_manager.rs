use sprite::{Sprite, SpriteData, SpriteCache};
use types::Point;
use sdl2::render::Renderer;
use camera::Camera;

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

    pub fn draw(&self, index: usize, pos: Point, mut r: &mut Renderer, c: &Camera) {
        if index > self.sprites.len() {
            println!("invalid frame for player: {}", index);

            //TODO rewrite this to render a rect instead of stupidly
            //     relying on the first sprite being there
            self.sprites[0].draw(pos, r, c);
            return;
        }

        self.sprites[index].draw(pos, r, c);
    }
}