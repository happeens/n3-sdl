use sdl2::pixels::Color;
use sdl2::render::Renderer;

use camera::Camera;
use tilemap::Tilemap;

pub struct World {
    tilemap: Tilemap,
}

impl World {
    pub fn new(mut r: &mut Renderer) -> World {
        let tilemap = Tilemap::new(&mut r);

        World {
            tilemap: tilemap
        }
    }

    pub fn update(&mut self, dt: f64) {
    }

    pub fn draw(&self, mut r: &mut Renderer, c: &Camera) {
        self.tilemap.draw(r, c);
    }
}
