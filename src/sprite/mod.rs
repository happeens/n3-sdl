pub mod spritecache;

use std::cell::RefCell;
use std::rc::Rc;

use sdl2::render::Texture;

use types::{Point, Size};

#[derive(Clone)]
pub struct Sprite {
    name: String,
    pos: Point,
    size: Size,
    src: Point,
    src_size: Size,
    tex: Rc<RefCell<Texture>>
}

impl Sprite {
    pub fn new(name: &str, pos: Point, size: Size, src: Point, src_size: Size, tex: Rc<RefCell<Texture>>) -> Sprite {
        Sprite {
            name: String::from(name),
            pos: pos,
            size: size,
            src: src,
            src_size: src_size,
            tex: tex
        }
    }
}
