mod tileset;

mod layer;
mod tile;
mod object;

use sdl2::render::{Texture, Renderer};
use sdl2::image::LoadTexture;

use std::cell::RefCell;
use std::rc::Rc;

use types::{Size, Point};
use camera::Camera;
use context::Context;

use self::layer::{TileLayer, LayerData, ObjectLayer};
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
    layers: Vec<LayerData>,
    properties: Option<TilemapConfigData>,
}

#[derive(Serialize, Deserialize, Debug)]
struct TilemapConfigData {
    #[serde(rename="entitylayer")]
    entity_layer: u16
}

pub struct Tilemap {
    width: u16,
    height: u16,
    tilesize: Size,
    bg_layers: Vec<TileLayer>,
    fg_layers: Vec<TileLayer>,
    object_layers: Vec<ObjectLayer>,
    tilesets: Vec<Tileset>,
}

impl Tilemap {
    pub fn new(mut ctx: &mut Context) -> Tilemap {
        let data: TilemapData = super::util::load_data("tilemap-small-0.json").unwrap();
        let tilesize = Size::new(data.tilewidth, data.tileheight);
        let mut entity_layer = 0;
        if let Some(props) = data.properties {
            entity_layer = props.entity_layer;
        }

        let mut tilesets = Vec::new();
        for td in &data.tilesets {
            if td.is_image_set() {
                continue;
            }

            tilesets.push(Tileset::new(&td, ctx));
        }

        let mut bg_layers = Vec::new();
        let mut fg_layers = Vec::new();
        let mut object_layers = Vec::new();

        let mut current = 0;
        for tl in data.layers.iter() {
            current += 1;

            if tl.is_object_layer() {
                object_layers.push(ObjectLayer::new(&tl));
                continue;
            }

            if current < entity_layer {
                bg_layers.push(TileLayer::new(&tl, &tilesets, &tilesize));
                continue;
            }

            fg_layers.push(TileLayer::new(&tl, &tilesets, &tilesize));
        }

        Tilemap {
            width: data.width,
            height: data.height,
            tilesize: tilesize,
            bg_layers: bg_layers,
            fg_layers: fg_layers,
            object_layers: object_layers,
            tilesets: tilesets,
        }
    }

    pub fn draw_fg(&self, mut ctx: &mut Context) {
        for layer in self.fg_layers.iter() {
            layer.draw(ctx);
        }
    }

    pub fn draw_bg(&self, mut ctx: &mut Context) {
        for layer in self.bg_layers.iter() {
            layer.draw(ctx);
        }
    }
}
