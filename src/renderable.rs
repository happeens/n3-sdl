use sdl2::pixels::Color;
use sdl2::render::Renderer;

use types::{Point, Size};
use camera::Camera;

#[derive(Debug, Copy, Clone)]
pub struct Renderable {
    pub pos: Point,
    pub anchor: Point,
    pub size: Size,
    pub color: Color,
}

impl Renderable {
    pub fn new(pos: Point, size: Size, color: Color) -> Renderable {
        Renderable {
            pos: pos,
            anchor: Point::new(size.w / 2.0, size.h / 2.0),
            size: size,
            color: color,
        }
    }

    pub fn draw(&self, r: &mut Renderer, c: &Camera) {
        r.set_draw_color(self.color);
        let pos = self.pos - c.get_pos() - self.anchor;
        let _ = r.fill_rect(pos.to_sdl_rect(self.size));
    }
}
