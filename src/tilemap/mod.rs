extern crate serde_json;

use sdl2::render::{Texture, Renderer};
use sdl2::image::LoadTexture;

use std::cell::RefCell;
use std::rc::Rc;

use std::fs::File;
use std::path::Path;
use std::collections::HashMap;

use std::io::Cursor;
use byteorder::{LittleEndian, ReadBytesExt};

use std::io::prelude::*;

use types::{Point, Size};

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

#[derive(Serialize, Deserialize, Debug)]
pub struct TilelayerData {
    data: String,
    encoding: String,
    height: u16,
    width: u16,
    opacity: f64,
    visible: bool,
    x: u16,
    y: u16,
}

pub struct Tilelayer {
    width: u16,
    height: u16,
    opacity: f64,
    visible: bool,
    data: Vec<Option<Tile>>,
}

pub struct Tile {
    src: Point,
    size: Size,
    tex: Rc<RefCell<Texture>>,
}

pub struct Tileset {
    firstgid: u32,
    tilesize: Size,
    columns: u32,
    tex: Rc<RefCell<Texture>>,
}

pub struct Tilemap {
    width: u16,
    height: u16,
    tilesize: Size,
    layers: Vec<Tilelayer>,
    tilesets: Vec<Tileset>,
}

impl Tilemap {
    pub fn new(r: &mut Renderer) -> Tilemap {
        let mut file = File::open(Path::new("assets/testmap.jmap")).unwrap();
        let mut content = String::new();
        let _ = file.read_to_string(&mut content);

        let data: TilemapData = serde_json::from_str(&content).unwrap();

        let mut tilesets = Vec::new();
        for td in &data.tilesets {
            let mut path = String::from("assets/");
            path.push_str(&td.image);
            let tex = r.load_texture(Path::new(&path)).unwrap();
            tilesets.push(Tileset {
                firstgid: td.firstgid,
                tilesize: Size::new(td.tilewidth as f64, td.tileheight as f64),
                columns: td.columns,
                tex: Rc::new(RefCell::new(tex))
            });
        }

        let mut layers = Vec::new();
        for tl in &data.layers {
            // decode data from base64
            use base64::decode;
            let bytes = decode(&tl.data).unwrap();
            let size = (tl.width * tl.height) as usize;

            // data should be interpreted as a u32 array
            // with little endian byte ordering
            assert!(size * 4 == bytes.len());
            let mut fields = Vec::with_capacity(size);
            for field in bytes.chunks(4) {
                let mut c = Cursor::new(field);
                let gid = c.read_u32::<LittleEndian>().unwrap();
                fields.push(get_tile(gid, &tilesets));
            }

            let layer = Tilelayer {
                width: tl.width,
                height: tl.height,
                opacity: tl.opacity,
                visible: tl.visible,
                data: fields,
            };
            layers.push(layer);
        }

        Tilemap {
            width: data.width,
            height: data.height,
            tilesize: Size::new(data.tilewidth, data.tileheight),
            layers: layers,
            tilesets: tilesets,
        }
    }

    pub fn draw(&self, r: &mut Renderer) {
        for layer in &self.layers {
            let mut counter = 0;
            for y in 0..layer.height {
                for x in 0..layer.width {
                    if let Some(ref tile) = layer.data[counter] {
                        if counter >= layer.data.len() { break; }
                        let dest = Point::new(
                            x as f64 * self.tilesize.w(),
                            y as f64 * self.tilesize.h()
                        );

                        r.copy(&mut tile.tex.borrow_mut(),
                               Some(tile.src.to_sdl_rect(tile.size)),
                               Some(dest.to_sdl_rect(self.tilesize)));
                    }
                    counter += 1;
                }
            }
        }
    }

}

fn get_tile(gid: u32, tilesets: &Vec<Tileset>) -> Option<Tile> {
    if gid == 0 { return None; }
    let mut tileset_id = 0;

    for (i, ts) in tilesets.iter().enumerate() {
        if ts.firstgid <= gid { continue; }
        tileset_id = i - 1;
        break;
    }

    let tileset = &tilesets[tileset_id];
    let gid = gid - tileset.firstgid;
    let (mut x, mut y) = (0.0, 0.0); 
    if gid != 0 {
        let offset = gid % tileset.columns;
        x = offset as f64 * tileset.tilesize.w();
        let row: u32 = gid / tileset.columns;
        y = row as f64 * tileset.tilesize.h();
    }

    let src = Point::new(x, y);

    Some(Tile {
        src: src,
        size: tileset.tilesize,
        tex: tileset.tex.clone(),
    })
}
