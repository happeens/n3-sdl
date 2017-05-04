use super::tileset::Tileset;
use super::tile::Tile;
use super::object::{ObjectData, TileObject};

use sdl2::render::{Texture, Renderer};
use sdl2::image::LoadTexture;

use types::{Size, Point, RenderInfo};
use camera::Camera;
use context::Context;

#[derive(Serialize, Deserialize, Debug)]
pub struct LayerData {
    #[serde(rename="type")]
    layer_type: String,

    // common fields
    pub name: String,
    height: u16,
    width: u16,
    opacity: f64,
    visible: bool,
    x: u16,
    y: u16,

    // tilelayer fields
    encoding: Option<String>,
    data: Option<String>,

    // objectlayer fields
    draworder: Option<String>,
    objects: Option<Vec<ObjectData>>,
}

impl LayerData {
    pub fn is_object_layer(&self) -> bool {
        self.layer_type == "objectgroup"
    }
}

pub struct TileLayer {
    width: u16,
    height: u16,
    opacity: f64,
    visible: bool,
    tilesize: Size,
    tiles: Vec<Tile>,
}

impl TileLayer {
    pub fn new(data: &LayerData,
               tilesets: &Vec<Tileset>,
               tilesize: &Size) -> TileLayer {
        // decode data from base64
        use base64::decode;
        let bytes = decode(&data.data.as_ref().unwrap()).unwrap();
        let size = (data.width * data.height) as usize;
        assert!(size * 4 == bytes.len());

        // data should be interpreted as a u32 array
        // with little endian byte ordering
        use byteorder::{LittleEndian, ReadBytesExt};
        use std::io::Cursor;
        let mut fields = Vec::with_capacity(size);
        for field in bytes.chunks(4) {
            let mut c = Cursor::new(field);
            let gid = c.read_u32::<LittleEndian>().unwrap();
            fields.push(gid);
        }

        let mut tiles = Vec::with_capacity(size);
        let mut counter = 0;
        for y in 0..data.height {
            for x in 0..data.width {
                if counter >= fields.len() { break; }
                let pos = Point::new(x as f64 * tilesize.w,
                                     y as f64 * tilesize.h);

                if let Some(tile) = Tile::from_gid(fields[counter],
                                                   &tilesets,
                                                   pos,
                                                   tilesize.clone()) {
                    tiles.push(tile);
                }
                counter += 1;
            }
        }

        TileLayer {
            width: data.width,
            height: data.height,
            opacity: data.opacity,
            visible: data.visible,
            tilesize: tilesize.clone(),
            tiles: tiles
        }
    }

    pub fn draw(&self, z: f64, ctx: &mut Context) {
        for tile in &self.tiles {
            tile.draw(z, ctx);
        }
    }
}

pub struct ObjectLayer {
    name: String,
    width: u16,
    height: u16,
    opacity: f64,
    visible: bool,
    objects: Vec<TileObject>,
}

impl ObjectLayer {
    pub fn new(data: &LayerData) -> ObjectLayer {
        let mut objects = Vec::new();

        for od in data.objects.as_ref().unwrap().iter() {
            objects.push(TileObject::new(od));
        }

        ObjectLayer {
            name: data.name.to_owned(),
            width: data.width,
            height: data.height,
            opacity: data.opacity,
            visible: data.visible,
            objects: objects
        }
    }

    pub fn draw(&self, ctx: &mut Context) {
        for object in &self.objects {
            object.draw(ctx);
        }
    }
}