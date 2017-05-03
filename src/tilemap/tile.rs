use sdl2::render::{Texture, Renderer};

use std::cell::{RefCell, RefMut};
use std::rc::Rc;

use super::tileset::Tileset;

use camera::Camera;
use types::{Point, Size, RenderInfo};
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
    //TODO figure out if this has a performance impact
    //     or if there's a better way to do it in general
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

    pub fn get_render_info(&self) -> RenderInfo {
        RenderInfo::Texture {
            pos: self.pos,
            size: self.size,
            src: self.src,
            src_size: self.src_size,
            tex: self.tex.clone()
        }
    }
}
