use sdl2::render::{Texture, Renderer};
use sdl2::image::LoadTexture;

use super::tile::TileData;

use std::collections::HashMap;
use std::cell::{RefCell, RefMut};
use std::rc::Rc;

use context::Context;

use std::path::Path;
use types::{Size, Point};

#[derive(Serialize, Deserialize, Debug)]
pub struct TilesetData {
    columns: u32,
    firstgid: u32,
    margin: u16,
    name: String,
    spacing: u16,
    tilecount: u16,
    tileheight: u16,
    tilewidth: u16,

    // single image tileset
    image: Option<String>,
    imageheight: Option<u16>,
    imagewidth: Option<u16>,

    //TODO handle image set loading
    // image collection tileset
    tiles: Option<HashMap<String, TileData>>,
}

impl TilesetData {
    pub fn is_image_set(&self) -> bool {
        self.tiles.is_some()
    }
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
            tex: ctx.load_texture(data.image.as_ref().unwrap())
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
