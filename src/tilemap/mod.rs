use types::Size;
use context::Context;

use self::layer::{TileLayer, LayerData, ObjectLayer};
use self::tileset::{Tileset, TilesetData, Imageset};

mod tileset;

mod layer;
mod tile;
mod object;

#[derive(Serialize, Deserialize, Debug)]
pub struct TilemapData {
    height: u16,
    width: u16,
    nextobjectid: u16,
    orientation: String,
    renderorder: String,
    tileheight: f32,
    tilewidth: f32,
    version: u16,
    tilesets: Vec<TilesetData>,
    layers: Vec<LayerData>
}

pub struct Tilemap {
    _width: u16,
    _height: u16,
    bg_layers: Vec<TileLayer>,
    fg_layers: Vec<TileLayer>,
    object_layers: Vec<ObjectLayer>,
    _tilesets: Vec<Tileset>,
    _imagesets: Vec<Imageset>
}

impl Tilemap {
    pub fn new(mut ctx: &mut Context) -> Tilemap {
        use util::load_data;
        let data: TilemapData = load_data("tilemap-small-0.json").unwrap();
        let tilesize = Size::new(data.tilewidth, data.tileheight);

        let mut tilesets = Vec::new();
        let mut imagesets = Vec::new();
        for td in &data.tilesets {
            if td.is_image_set() {
                imagesets.push(Imageset::new(&td, ctx));
                continue;
            }

            tilesets.push(Tileset::new(&td, ctx));
        }

        let mut bg_layers = Vec::new();
        let mut fg_layers = Vec::new();
        let mut object_layers = Vec::new();

        let mut parsing_background = true;
        for tl in data.layers.iter() {
            if tl.is_object_layer() {
                if tl.name == "entities" { parsing_background = false; }
                object_layers.push(ObjectLayer::new(&tl, &imagesets));
                continue;
            }

            if parsing_background {
                bg_layers.push(TileLayer::new(&tl, &tilesets, &tilesize));
                continue;
            }

            fg_layers.push(TileLayer::new(&tl, &tilesets, &tilesize));
        }

        Tilemap {
            _width: data.width,
            _height: data.height,
            bg_layers: bg_layers,
            fg_layers: fg_layers,
            object_layers: object_layers,
            _tilesets: tilesets,
            _imagesets: imagesets
        }
    }

    pub fn draw(&self, mut ctx: &mut Context) {
        //TODO calculate z values according to map size
        for layer in &self.bg_layers { layer.draw(-10000.0, ctx); }
        for layer in &self.fg_layers { layer.draw(10000.0, ctx); }
        for object_layer in &self.object_layers { object_layer.draw(ctx); }
    }
}
