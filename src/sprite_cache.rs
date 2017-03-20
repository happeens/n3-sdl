use sdl2::render::{Renderer, Texture};
use sdl2::image::LoadTexture;
use sdl2::rect::Rect;

use std::path::Path;

use sprite::Sprite;

pub struct SpriteCache {
    textures: Vec<Texture>,
    sprites: Vec<Sprite>,
}

impl SpriteCache {
    pub fn new() -> SpriteCache {
        let textures = Vec::new();
        let sprites = Vec::new();

        SpriteCache {
            textures: textures,
            sprites: sprites,
        }
    }

    pub fn load_sheet(&mut self, path: &str, r: &mut Renderer) {
        let texture = r.load_texture(Path::new(path)).unwrap();
        let new_id = self.textures.len();
        self.sprites.push(Sprite::new("grass", new_id, Rect::new(0, 0, 16, 16)));
        self.textures.push(texture);
    }

    pub fn draw_sprite(&self, sprite: &Sprite, dest: Rect, r: &mut Renderer) {
        if sprite.sheet_id >= self.textures.len() {
            println!("invalid texture id!");
            return;
        }

        r.copy(&self.textures[sprite.sheet_id], Some(sprite.rect), Some(dest));
    }

    pub fn get_sprite(&self, name: &str) -> Option<Sprite> {
        for sprite in self.sprites.iter() {
            if sprite.name == name {
                return Some(sprite.clone());
            }
        }

        None
    }
}
