use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

use context::Context;
use types::{Size, Texture};

use super::tile::TileData;

#[derive(Serialize, Deserialize, Debug)]
pub struct TilesetData {
    columns: u32,
    firstgid: u32,
    margin: u16,
    name: String,
    spacing: u16,
    tilecount: u32,
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
    pub firstgid: u32,
    pub tilecount: u32,
    pub tilesize: Size,
    pub columns: u32,
    tex: Rc<RefCell<Texture>>,
}

impl Tileset {
    pub fn new(data: &TilesetData, ctx: &mut Context) -> Tileset {
        Tileset {
            firstgid: data.firstgid,
            tilecount: data.tilecount,
            tilesize: Size::new(data.tilewidth as f64, data.tileheight as f64),
            columns: data.columns,
            tex: ctx.load_texture(data.image.as_ref().unwrap())
        }
    }
    
    pub fn get_tex(&self) -> Rc<RefCell<Texture>> {
        self.tex.clone()
    }
}

pub struct Imageset {
    pub firstgid: u32,
    pub tilecount: u32,
    images: HashMap<String, Rc<RefCell<Texture>>>,
}

impl Imageset {
    pub fn new(data: &TilesetData, ctx: &mut Context) -> Imageset {
        let mut images = HashMap::new();
        for (name, tile_data) in data.tiles.as_ref().unwrap() {
            images.insert(name.to_owned(), ctx.load_texture(&tile_data.image));
        }

        Imageset {
            firstgid: data.firstgid,
            tilecount: data.tilecount,
            images: images
        }
    }

    pub fn get_tex_for_gid(&self, gid: u32) -> Option<Rc<RefCell<Texture>>> {
        let gid = gid - self.firstgid;
        match self.images.get(&gid.to_string()) {
            Some(image) => Some(image.clone()),
            None => None
        }
    }
}
