use std::rc::Rc;
use std::cell::RefCell;

use types::{Point, Size, RenderInfo, Texture};
use context::Context;

use super::tileset::Imageset;

#[derive(Serialize, Deserialize, Debug)]
pub struct ObjectData {
    width: u16,
    height: u16,
    x: f32,
    y: f32,
    visible: bool,
    id: u16,
    gid: u32
}

pub struct TileObject {
    pos: Point,
    size: Size,
    src: Point,
    src_size: Size,
    tex: Rc<RefCell<Texture>>
}

impl TileObject {
    pub fn new(data: &ObjectData, imagesets: &Vec<Imageset>) -> Option<TileObject> {
        let gid = data.gid;
        if gid == 0 { return None; }
        let mut imageset_id = 0;

        for (i, is) in imagesets.iter().enumerate() {
            if gid >= is.firstgid && gid < is.firstgid + is.tilecount {
                imageset_id = i;
            }
        }

        let tex = imagesets[imageset_id].get_tex_for_gid(gid);
        if !tex.is_some() { return None; }
        let tex = tex.unwrap();
        let tex_info = tex.borrow().query();

        Some(TileObject { 
            pos: Point::new(data.x, data.y - data.height as f32),
            size: Size::new(data.width as f32, data.height as f32),
            src: Point::new(0.0, 0.0),
            src_size: Size::new(tex_info.width as f32, tex_info.height as f32),
            tex: tex.clone()
        })
    }

    pub fn draw(&self, ctx: &mut Context) {
        let z = self.pos.y + self.size.h / 2.0;
        ctx.render(RenderInfo::texture(self.pos, self.size,
                                       self.src, self.src_size,
                                       z, self.tex.clone()));
    }
}
