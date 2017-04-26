pub mod spritecache;

use std::cell::RefCell;
use std::rc::Rc;

use sdl2::render::{Texture, Renderer};

use types::Size;
use types::CgPoint as Point;
use camera::Camera;

#[derive(Clone)]
pub struct Sprite {
    pub name: String,
    size: Size,
    src: Point,
    src_size: Size,
    tex: Rc<RefCell<Texture>>
}

impl Sprite {
    pub fn new(name: &str, size: Size, src: Point, src_size: Size, tex: Rc<RefCell<Texture>>) -> Sprite {
        Sprite {
            name: String::from(name),
            size: size,
            src: src,
            src_size: src_size,
            tex: tex
        }
    }

    pub fn draw(&self, pos: Point, r: &mut Renderer, c: &Camera) {
        let dest = pos - c.get_pos();
        let _ = r.copy(&mut self.tex.borrow_mut(),
                       Some(self.src.to_sdl_rect(self.src_size)),
                       Some(dest.to_sdl_rect(self.size)));
    }
}
