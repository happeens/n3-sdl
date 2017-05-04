use std::rc::Rc;
use std::cell::RefCell;

use sdl2::rect::Rect as SdlRect;

pub use sdl2::pixels::Color as Color;
pub use sdl2::render::Texture as Texture;

pub type Point = super::cgmath::Point2<f64>;
pub type Vec2 = super::cgmath::Vector2<f64>;

#[derive(Clone)]
pub struct RenderInfo {
    pub pos: Point,
    pub size: Size,
    pub z: f64,
    pub renderable: Renderable,
}

#[derive(Clone)]
pub enum Renderable {
    //TODO figure out if this has a performance impact using RcRefCell
    //     for the texture here or if there's a better way to do it in general
    Texture { src: Point, src_size: Size, tex: Rc<RefCell<Texture>> },
    Rect { color: Color },
}

impl RenderInfo {
    pub fn texture(pos: Point, size: Size,
                   src: Point, src_size: Size,
                   z: f64, tex: Rc<RefCell<Texture>>) -> RenderInfo {
        RenderInfo {
            pos: pos,
            size: size,
            z: z,
            renderable: Renderable::Texture { src: src, src_size: src_size, tex: tex }
        }
    }

    pub fn rect(pos: Point, size: Size, z: f64, color: Color) -> RenderInfo {
        RenderInfo {
            pos: pos,
            size: size,
            z: z,
            renderable: Renderable::Rect { color: color }
        }
    }
}

#[derive(Copy, Clone)]
pub enum Direction {
    Down,
    Up,
    Left,
    Right
}

pub fn to_sdl_rect(p: Point, s: Size) -> SdlRect {
    SdlRect::new(p.x as i32, p.y as i32, s.w as u32, s.h as u32)
}

#[derive(PartialEq, Eq, Debug, Hash, Copy, Clone)]
pub enum KeyAction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Copy, Clone, Debug)]
pub struct Size {
    pub w: f64,
    pub h: f64,
}

impl Size {
    pub fn new(w: f64, h: f64) -> Size {
        Size { w: w, h: h }
    }

    pub fn to_point(&self) -> Point {
        Point::new(self.w, self.h)
    }
}
