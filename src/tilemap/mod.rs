mod tilelayer;
mod tileset;
mod tile;

use sdl2::render::{Texture, Renderer};
use sdl2::image::LoadTexture;

use std::cell::RefCell;
use std::rc::Rc;

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
    bg_layers: Vec<Tilelayer>,
    fg_layers: Vec<Tilelayer>,
    tilesets: Vec<Tileset>,
}

impl Tilemap {
    pub fn new(mut r: &mut Renderer) -> Tilemap {
        let data: TilemapData = super::util::load_data("testmap.json").unwrap();
        let tilesize = Size::new(data.tilewidth, data.tileheight);

        let mut tilesets = Vec::new();
        for td in &data.tilesets {
            tilesets.push(Tileset::new(&td, r));
        }

        let mut bg_layers = Vec::new();
        let mut fg_layers = Vec::new();
        for tl in &data.layers {
            //TODO decide this based on layer properties
            println!("layer name: {}", tl.name);
            if tl.name == "ground" {
                println!("bg layer found!");
                bg_layers.push(Tilelayer::new(&tl, &tilesets, &tilesize));
            } else {
                fg_layers.push(Tilelayer::new(&tl, &tilesets, &tilesize));
            }
        }

        Tilemap {
            width: data.width,
            height: data.height,
            tilesize: tilesize,
            bg_layers: bg_layers,
            fg_layers: fg_layers,
            tilesets: tilesets,
        }
    }

    pub fn draw_background(&self, mut r: &mut Renderer, c: &Camera) {
        for layer in self.bg_layers.iter() {
            layer.draw(r, c);
        }
    }

    pub fn draw_foreground(&self, mut r: &mut Renderer, c: &Camera) {
        for layer in self.fg_layers.iter() {
            layer.draw(r, c);
        }
    }
}
