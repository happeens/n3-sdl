use sdl2::pixels::Color;
use sdl2::render::Renderer;

use camera::Camera;
use types::{Point, Size};

const TILE_WIDTH: u32 = 40;
const TILE_HEIGHT: u32 = 40;

struct Tile {
    color: Color,
    meta: u16,
}

pub struct World {
    fields: Vec<Vec<Tile>>,
}

impl World {
    pub fn new() -> World {
        let mut fields = Vec::new();
        for y in 0..10 {
            let mut row = Vec::new();
            for x in 0..10 {
                if x == 0 || x == 9 || y == 0 || y == 9 {
                    row.push(Tile { color: Color::RGB(255, 0, 0), meta: 1 });
                } else {
                    row.push(Tile { color: Color::RGB(0, 255, 0), meta: 0 });
                }
            }
            fields.push(row);
        }

        World {
            fields: fields,
        }
    }

    pub fn update(&mut self, dt: f64) {
    }

    pub fn draw(&self, r: &mut Renderer, c: &Camera) {
        let tile_size = Size::new(TILE_WIDTH as f64, TILE_HEIGHT as f64);

        for (y, row) in self.fields.iter().enumerate() {
            for (x, field) in row.iter().enumerate() {
                let field_x = x as u32 * TILE_WIDTH;
                let field_y = y as u32 * TILE_HEIGHT;
                let field_pos = Point::new(field_x as f64, field_y as f64) - c.get_pos();

                r.set_draw_color(field.color);
                let _ = r.fill_rect(field_pos.to_sdl_rect(tile_size));

                // draw bounds
                r.set_draw_color(Color::RGB(0, 0, 0));
                let _ = r.draw_rect(field_pos.to_sdl_rect(tile_size));
            }
        }
    }

    pub fn to_tile_pos(&self, pos: Point) -> Point {
    }

    pub fn check_pos_collides(&self, pos: Point) -> bool {
        let tile_pos = 
    }
}
