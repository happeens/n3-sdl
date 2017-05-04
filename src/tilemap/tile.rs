use sdl2::render::{Texture, Renderer};

use std::cell::{RefCell, RefMut};
use std::rc::Rc;

use super::tileset::Tileset;

use context::Context;
use types::{Point, Size, RenderInfo};
use types::to_sdl_rect;

#[derive(Serialize, Deserialize, Debug)]
pub struct TileData {
    pub image: String
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
            if gid >= ts.firstgid && gid < ts.firstgid + ts.tilecount {
                tileset_id = i;
            }
        }

        let tileset = &tilesets[tileset_id];
        let gid = gid - tileset.firstgid;
        let (mut x, mut y) = (0.0, 0.0); 
        if gid != 0 {
            let size = tileset.tilesize;
            let cols = tileset.columns;

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
            src_size: tileset.tilesize,
            tex: tileset.get_tex(),
        })
    }

    pub fn draw(&self, z: f64, ctx: &mut Context) {
        ctx.render(RenderInfo::texture(self.pos, self.size,
                                       self.src, self.src_size,
                                       z, self.tex.clone()));
    }
}
