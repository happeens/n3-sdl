use super::tileset::Tileset;
use super::tile::Tile;

use sdl2::render::{Texture, Renderer};
use sdl2::image::LoadTexture;

use types::{Point, Size};
use camera::Camera;

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
    tilesize: Size,
    tiles: Vec<Option<Tile>>,
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
                let pos = Point::new(x as f64 * tilesize.w(),
                                     y as f64 * tilesize.h());

                tiles.push(Tile::from_gid(fields[counter], &tilesets, pos, tilesize.clone()));
                counter += 1;
            }
        }

        Tilelayer {
            width: data.width,
            height: data.height,
            opacity: data.opacity,
            visible: data.visible,
            tilesize: tilesize.clone(),
            tiles: tiles,
        }
    }

    pub fn draw(&self, mut r: &mut Renderer, c: &Camera) {
        for tile in &self.tiles {
            if let &Some(ref t) = tile {
                t.draw(r, c);
            }
        }
        // if let Some(ref tile) = self.data[counter] {
        //     let _ = r.copy(&mut tile.get_tex(),
        //            Some(tile.get_src().to_sdl_rect(tile.get_size())),
        //            Some(dest.to_sdl_rect(self.tilesize)));
        // }
    }
}
