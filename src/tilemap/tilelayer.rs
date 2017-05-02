use super::tileset::Tileset;
use super::tile::Tile;

use sdl2::render::{Texture, Renderer};
use sdl2::image::LoadTexture;

use types::{Size, Point};
use camera::Camera;
use context::Context;

#[derive(Serialize, Deserialize, Debug)]
pub struct TilelayerData {
    pub name: String,
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
    tilesize: Size,
    tiles: Vec<Tile>,
}

impl Tilelayer {
    pub fn new(data: &TilelayerData,
               tilesets: &Vec<Tileset>,
               tilesize: &Size) -> Tilelayer {
        // decode data from base64
        use base64::decode;
        let bytes = decode(&data.data).unwrap();
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

        Tilelayer {
            width: data.width,
            height: data.height,
            opacity: data.opacity,
            visible: data.visible,
            tilesize: tilesize.clone(),
            tiles: tiles
        }
    }

    pub fn draw(&self, ctx: &mut Context) {
        //TODO clean up this mess
        for tile in self.tiles.iter() {
            let dest = tile.get_pos();
            ctx.draw_texture(dest, tile);
        }
    }
}
