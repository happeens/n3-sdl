use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

use sdl2::render::Renderer;
use sdl2::image::LoadTexture;

use types::{Size, Point};

use super::Sprite;

pub struct SpriteCache {
    sprites: Vec<Sprite>,
}

#[derive(Serialize, Deserialize, Debug)]
struct SpritesheetData {
    name: String,
    size: SizeData,
    o_size: SizeData,
    pos: PointData,
    offset: PointData
}

#[derive(Serialize, Deserialize, Debug)]
struct PointData {
    x: f64,
    y: f64
}

#[derive(Serialize, Deserialize, Debug)]
struct SizeData {
    width: f64,
    height: f64
}

impl SpriteCache {
    pub fn new() -> SpriteCache {
        let sprites = Vec::new();

        SpriteCache {
            sprites: sprites,
        }
    }

    pub fn load_sheet(&mut self, name: &str, r: &mut Renderer) {
        let mut path = String::from("assets/");
        path.push_str(name);

        let mut path_meta = String::from(path.clone());
        let mut path_img = String::from(path.clone());

        path_meta.push_str(".json");
        path_img.push_str(".png");

        // load metadata
        let mut file = File::open(Path::new(&path_meta)).unwrap();
        let mut content = String::new();
        let _ = file.read_to_string(&mut content);

        let data: Vec<SpritesheetData> = super::super::serde_json::from_str(&content).unwrap();

        let tex = r.load_texture(Path::new(&path_img)).unwrap();
        let tex = Rc::new(RefCell::new(tex));

        for sd in &data {
            let sprite = Sprite::new(&sd.name,
                                     Size::new(sd.size.width, sd.size.height),
                                     Point::new(sd.pos.x, sd.pos.y),
                                     Size::new(sd.size.width, sd.size.height),
                                     tex.clone());
            self.sprites.push(sprite);
        }
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
