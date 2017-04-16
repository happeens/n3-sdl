use sdl2::pixels::Color;
use sdl2::render::Renderer;

use camera::Camera;
use types::{Point, Size, TilePos};
use sprite_cache::SpriteCache;
use tilemap::Tilemap;

const TILE_WIDTH: u32 = 40;
const TILE_HEIGHT: u32 = 40;

#[derive(Copy, Clone)]
pub struct Tile {
    color: Color,
    meta: u16,
}

pub struct World {
    tilemap: Tilemap,
    highlighted: TilePos,
}

impl World {
    pub fn new(mut r: &mut Renderer) -> World {
        let tilemap = Tilemap::new(&mut r);

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
            tilemap: tilemap,
            highlighted: TilePos::new(0, 0),
        }
    }

    pub fn update(&mut self, dt: f64) {
    }

    pub fn draw(&self, mut r: &mut Renderer, s: &mut SpriteCache, c: &Camera) {
        self.tilemap.draw(r, c);
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

    pub fn from_screen_pos(&self, screen_x: i32, screen_y: i32, camera: &Camera) -> TilePos {
        let mut world_pos = Point::new(screen_x as f64, screen_y as f64);
        world_pos = world_pos + camera.get_pos();
        self.to_tile_pos(world_pos)
    }

    pub fn check_pos_collides(&self, pos: Point) -> bool {
        false
    }

    pub fn set_highlighted(&mut self, pos: TilePos) {
        self.highlighted = pos;
    }
}
