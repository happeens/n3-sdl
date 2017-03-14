use sdl2::pixels::Color;
use sdl2::render::Renderer;

use camera::Camera;
use types::{Point, Size, TilePos};

const TILE_WIDTH: u32 = 40;
const TILE_HEIGHT: u32 = 40;

#[derive(Copy, Clone)]
pub struct Tile {
    color: Color,
    meta: u16,
}

pub struct World {
    fields: Vec<Vec<Tile>>,
    highlighted: TilePos,
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
            highlighted: TilePos::new(0, 0),
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
                if x == self.highlighted.x() as usize && y == self.highlighted.y() as usize {
                    r.set_draw_color(Color::RGB(255, 255, 0));
                }

                let _ = r.fill_rect(field_pos.to_sdl_rect(tile_size));

                // draw bounds
                r.set_draw_color(Color::RGB(0, 0, 0));
                let _ = r.draw_rect(field_pos.to_sdl_rect(tile_size));
            }
        }
    }

    pub fn to_tile_pos(&self, pos: Point) -> TilePos {
        let tile_size = Size::new(TILE_WIDTH as f64, TILE_HEIGHT as f64);
        let tile_pos = pos / tile_size;
        TilePos::from_point(tile_pos)
    }

    pub fn from_tile_pos(&self, tile_pos: TilePos) -> Point {
        let tile_size = Size::new(TILE_WIDTH as f64, TILE_HEIGHT as f64);
        let pos_x = tile_pos.x() as f64 * tile_size.w() + tile_size.w() / 2.0;
        let pos_y = tile_pos.y() as f64 * tile_size.h() + tile_size.h() / 2.0;
        Point::new(pos_x, pos_y)
    }

    pub fn check_pos_collides(&self, pos: Point) -> bool {
        let tile_pos = self.to_tile_pos(pos);
        self.get_field(tile_pos).meta == 1
    }

    pub fn get_field(&self, pos: TilePos) -> Tile {
        self.fields[pos.x() as usize][pos.y() as usize]
    }

    pub fn set_highlighted(&mut self, pos: TilePos) {
        self.highlighted = pos;
    }
}
