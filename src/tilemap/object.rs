use types::{Point, Size};

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