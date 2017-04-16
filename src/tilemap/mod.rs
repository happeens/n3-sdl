extern crate serde_json;

mod tilelayer;
mod tileset;
mod tile;

use sdl2::render::{Texture, Renderer};
use sdl2::image::LoadTexture;

use std::cell::RefCell;
use std::rc::Rc;

use std::fs::File;
use std::path::Path;
use std::collections::HashMap;

use std::io::prelude::*;

use types::{Size, Point};
use camera::Camera;

use self::tilelayer::{Tilelayer, TilelayerData};
use self::tileset::{Tileset, TilesetData};
use self::tile::Tile;

#[derive(Serialize, Deserialize, Debug)]
pub struct TilemapData {
    height: u16,
    width: u16,
    nextobjectid: u16,
    orientation: String,
    renderorder: String,
    tileheight: f64,
    tilewidth: f64,
    version: u16,
    tilesets: Vec<TilesetData>,
    layers: Vec<TilelayerData>,
}

pub struct Tilemap {
    width: u16,
    height: u16,
    tilesize: Size,
    layers: Vec<Tilelayer>,
    tilesets: Vec<Tileset>,
}

impl Tilemap {
    pub fn new(mut r: &mut Renderer) -> Tilemap {
        let mut file = File::open(Path::new("assets/testmap.jmap")).unwrap();
        let mut content = String::new();
        let _ = file.read_to_string(&mut content);

        let data: TilemapData = serde_json::from_str(&content).unwrap();
        let tilesize = Size::new(data.tilewidth, data.tileheight);

        let mut tilesets = Vec::new();
        for td in &data.tilesets {
            tilesets.push(Tileset::new(&td, r));
        }

        let mut layers = Vec::new();
        for tl in &data.layers {
            layers.push(Tilelayer::new(&tl, &tilesets, &tilesize));
        }

        Tilemap {
            width: data.width,
            height: data.height,
            tilesize: tilesize,
            layers: layers,
            tilesets: tilesets,
        }
    }

    pub fn draw(&self, mut r: &mut Renderer, c: &Camera) {
        for layer in &self.layers {
            layer.draw(r, c);
        }
    }
}
