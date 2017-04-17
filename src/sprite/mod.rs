pub mod spritecache;

use std::cell::RefCell;
use std::rc::Rc;

use sdl2::render::{Texture, Renderer};

use types::{Point, Size};
use camera::Camera;

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

    pub fn draw(&self, r: &mut Renderer, c: &Camera) {
        let dest = self.pos - c.get_pos();
        let _ = r.copy(&mut self.tex.borrow_mut(),
                       Some(self.src.to_sdl_rect(self.src_size)),
                       Some(dest.to_sdl_rect(self.size)));
    }

    pub fn get_pos(&self) -> Point {
        self.pos
    }

    pub fn set_pos(&mut self, pos: Point) {
        self.pos = pos;
    }
}
