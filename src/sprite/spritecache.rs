use sdl2::render::{Renderer, Texture};
use sdl2::image::LoadTexture;
use sdl2::rect::Rect;

use std::path::Path;

use super::Sprite;

pub struct SpriteCache {
    sprites: Vec<Sprite>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SpritesheetData {
}

impl SpriteCache {
    pub fn new() -> SpriteCache {
        let sprites = Vec::new();

        SpriteCache {
            sprites: sprites,
        }
    }

    pub fn load_sheet(&mut self, path: &str, r: &mut Renderer) {
        //TODO
        println!("loading sheet: {}", path);
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
