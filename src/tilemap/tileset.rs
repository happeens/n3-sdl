use sdl2::render::{Texture, Renderer};
use sdl2::image::LoadTexture;

use std::cell::{RefCell, RefMut};
use std::rc::Rc;

use context::Context;

use std::path::Path;
use types::{Size, Point};

#[derive(Serialize, Deserialize, Debug)]
pub struct TilesetData {
    columns: u32,
    firstgid: u32,
    image: String,
    imageheight: u16,
    imagewidth: u16,
    margin: u16,
    name: String,
    spacing: u16,
    tilecount: u16,
    tileheight: u16,
    tilewidth: u16,
}

pub struct Tileset {
    firstgid: u32,
    tilesize: Size,
    columns: u32,
    tex: Rc<RefCell<Texture>>,
}

impl Tileset {
    pub fn new(data: &TilesetData, ctx: &mut Context) -> Tileset {
        Tileset {
            firstgid: data.firstgid,
            tilesize: Size::new(data.tilewidth as f64, data.tileheight as f64),
            columns: data.columns,
            tex: ctx.load_texture(&data.image)
        }
    }

    pub fn get_firstgid(&self) -> u32 {
        self.firstgid
    }

    pub fn get_tilesize(&self) -> Size {
        self.tilesize
    }

    pub fn get_columns(&self) -> u32 {
        self.columns
    }
    
    pub fn clone_tex(&self) -> Rc<RefCell<Texture>> {
        self.tex.clone()
    }
}
