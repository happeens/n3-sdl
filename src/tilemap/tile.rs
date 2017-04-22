use sdl2::render::{Texture, Renderer};

use std::cell::{RefCell, RefMut};
use std::rc::Rc;

use super::tileset::Tileset;

use camera::Camera;
use types::{Point, Size};

pub struct Tile {
    pos: Point,
    size: Size,
    src: Point,
    src_size: Size,
    tex: Rc<RefCell<Texture>>,
}

impl Tile {
    pub fn from_gid(gid: u32, tilesets: &Vec<Tileset>, pos: Point, size: Size) -> Option<Tile> {
        if gid == 0 { return None; }
        let mut tileset_id = 0;

        for (i, ts) in tilesets.iter().enumerate() {
            if ts.get_firstgid() <= gid { continue; }
            tileset_id = i - 1;
            break;
        }

        let tileset = &tilesets[tileset_id];
        let gid = gid - tileset.get_firstgid();
        let (mut x, mut y) = (0.0, 0.0); 
        if gid != 0 {
            let size = tileset.get_tilesize();
            let cols = tileset.get_columns();

            let offset = gid % cols;
            x = offset as f64 * size.w;
            let row: u32 = gid / cols;
            y = row as f64 * size.h;
        }

        let src = Point::new(x, y);

        Some(Tile {
            pos: pos,
            size: size,
            src: src,
            src_size: tileset.get_tilesize(),
            tex: tileset.clone_tex(),
        })
    }

    pub fn get_src(&self) -> Point {
        self.src
    }

    pub fn get_size(&self) -> Size {
        self.size
    }

    pub fn get_tex(&self) -> RefMut<Texture> {
        self.tex.borrow_mut()
    }

    pub fn draw(&self, r: &mut Renderer, c: &Camera) {
        let dest = self.pos - c.get_pos();
        let _ = r.copy(&mut self.tex.borrow_mut(),
                       Some(self.src.to_sdl_rect(self.src_size)),
                       Some(dest.to_sdl_rect(self.size)));
    }
}
