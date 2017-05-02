use sdl2::rect::Rect as SdlRect;
use sdl2::render::Texture;
use std::cell::RefMut;

use std::ops::{Add, Sub, Mul, Div};

pub type Point = super::cgmath::Point2<f64>;
pub type Vec2 = super::cgmath::Vector2<f64>;

pub trait Drawable {
    fn get_src(&self) -> Point;
    fn get_src_size(&self) -> Size;
    fn get_size(&self) -> Size;
    fn get_tex(&self) -> RefMut<Texture>;
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
