use sdl2::rect::Rect as SdlRect;
use sdl2::render::Texture;

pub use sdl2::pixels::Color as Color;

use std::ops::{Add, Sub, Mul, Div};
use std::rc::Rc;
use std::cell::RefCell;

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
    SdlRect::new(p.x.floor() as i32, p.y.floor() as i32, s.w.floor() as u32, s.h.floor() as u32)
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

#[derive(Debug, Copy, Clone)]
pub struct TilePos {
    pub x: u32,
    pub y: u32,
}

impl TilePos {
    pub fn new(x: u32, y: u32) -> TilePos {
        TilePos { x: x, y: y }
    }

    pub fn from_point(p: Point) -> TilePos {
        TilePos::new(p.x as u32, p.y as u32)
    }
}

impl Sub<TilePos> for TilePos {
    type Output = TilePos;

    fn sub(self, rhs: TilePos) -> TilePos {
        TilePos::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Size {
    pub fn new(w: f64, h: f64) -> Size {
        Size { w: w, h: h }
    }

    pub fn to_point(&self) -> Point {
        Point::new(self.w, self.h)
    }
}
