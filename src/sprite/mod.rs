mod sprite_cache;
pub type SpriteCache = sprite_cache::SpriteCache;

mod sprite_manager;
pub type SpriteManager = sprite_manager::SpriteManager;

use std::cell::{RefCell, RefMut};
use std::rc::Rc;

use sdl2::render::{Texture, Renderer};

use types::{Point, Size, RenderInfo};
use types::to_sdl_rect;
use camera::Camera;

#[derive(Serialize, Deserialize, Debug)]
pub struct SpriteData {
    name: String
}

#[derive(Clone)]
pub struct Sprite {
    pub name: String,
    size: Size,
    src: Point,
    src_size: Size,
    scale: f64,
    tex: Rc<RefCell<Texture>>
}

impl Sprite {
    pub fn new(name: &str, size: Size, src: Point, src_size: Size, tex: Rc<RefCell<Texture>>) -> Sprite {
        Sprite {
            name: String::from(name),
            size: size,
            src: src,
            src_size: src_size,
            scale: 1.0,
            tex: tex
        }
    }

    pub fn get_render_info(&self, pos: Point) -> RenderInfo {
        RenderInfo::Texture {
            pos: pos,
            size: self.size,
            src: self.src,
            src_size: self.src_size,
            tex: self.tex.clone()
        }
    }
}
