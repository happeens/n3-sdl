use sdl2::render::{Texture, Renderer};

use std::cell::{RefCell, RefMut};
use std::rc::Rc;

use super::tileset::Tileset;

use camera::Camera;
use types::{Point, Size, Drawable};
use types::to_sdl_rect;

#[derive(Serialize, Deserialize, Debug)]
pub struct TileData {
    image: String
}

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

    pub fn get_pos(&self) -> Point {
        self.pos
    }
}

impl Drawable for Tile {
    fn get_src(&self) -> Point {
        self.src
    }

    fn get_src_size(&self) -> Size {
        self.src_size
    }

    fn get_size(&self) -> Size {
        self.size
    }

    fn get_tex(&self) -> RefMut<Texture> {
        self.tex.borrow_mut()
    }
}
