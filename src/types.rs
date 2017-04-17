use sdl2::rect::Rect as SdlRect;

use std::ops::{Add, Sub, Mul, Div};

#[derive(PartialEq, Eq, Debug, Hash)]
pub enum KeyAction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Copy, Clone, Debug)]
pub struct Point {
    x: f64,
    y: f64,
}

#[derive(Copy, Clone, Debug)]
pub struct Size {
    w: f64,
    h: f64,
}

#[derive(Debug, Copy, Clone)]
pub struct TilePos {
    x: u32,
    y: u32,
}

impl TilePos {
    pub fn new(x: u32, y: u32) -> TilePos {
        TilePos { x: x, y: y }
    }

    pub fn from_point(p: Point) -> TilePos {
        TilePos::new(p.x() as u32, p.y() as u32)
    }

    pub fn x(&self) -> u32 {
        self.x
    }

    pub fn y(&self) -> u32 {
        self.y
    }
}

impl Sub<TilePos> for TilePos {
    type Output = TilePos;

    fn sub(self, rhs: TilePos) -> TilePos {
        TilePos::new(self.x - rhs.x(), self.y - rhs.y())
    }
}

impl Point {
    pub fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }

    pub fn to_sdl_rect(&self, s: Size) -> SdlRect {
        SdlRect::new(self.x as i32,
                     self.y as i32,
                     s.w as u32,
                     s.h as u32)
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn set_x(&mut self, x: f64) {
        self.x = x;
    }

    pub fn set_y(&mut self, y: f64) {
        self.y = y;
    }

    pub fn add_x(&mut self, x: f64) {
        self.x += x;
    }

    pub fn add_y(&mut self, y: f64) {
        self.y += y;
    }

    pub fn is_diag(&self) -> bool {
        self.x != 0.0 && self.y != 0.0
    }

    pub fn mult_diag(&mut self) {
        let mult = (0.5f64).sqrt();
        self.x *= mult;
        self.y *= mult;
    }

    pub fn round(&mut self) {
        self.x = self.x.round();
        self.y = self.y.round();
    }
}

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Point {
        Point::new(self.x + rhs.x(), self.y + rhs.y())
    }
}

impl Sub<Point> for Point {
    type Output = Point;

    fn sub(self, rhs: Point) -> Point {
        Point::new(self.x - rhs.x(), self.y - rhs.y())
    }
}

impl Mul<f64> for Point {
    type Output = Point;

    fn mul(self, rhs: f64) -> Point {
        Point::new(self.x * rhs, self.y * rhs)
    }
}

impl Div<Size> for Point {
    type Output = Point;

    fn div(self, rhs: Size) -> Point {
        Point::new(self.x / rhs.w(), self.y / rhs.h())
    }
}

impl Size {
    pub fn new(w: f64, h: f64) -> Size {
        Size { w: w, h: h }
    }

    pub fn to_point(&self) -> Point {
        Point::new(self.w, self.h)
    }

    pub fn w(&self) -> f64 {
        self.w
    }

    pub fn h(&self) -> f64 {
        self.h
    }
}
