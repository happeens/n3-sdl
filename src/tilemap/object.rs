use types::{Point, Size, RenderInfo, Color};
use context::Context;

#[derive(Serialize, Deserialize, Debug)]
pub struct ObjectData {
    width: u16,
    height: u16,
    x: f64,
    y: f64,
    visible: bool,
    id: u16
}

pub struct TileObject {
    size: Size,
    pos: Point,
}

impl TileObject {
    pub fn new(data: &ObjectData) -> TileObject {
        TileObject { 
            pos: Point::new(data.x, data.y),
            size: Size::new(data.width as f64, data.height as f64)
        }
    }

    pub fn draw(&self, ctx: &mut Context) {
        ctx.render(RenderInfo::rect(self.pos, self.size,
                                    self.pos.y, Color::RGB(255, 255, 0)));
    }
}
