use sdl2::rect::Rect;

#[derive(Clone)]
pub struct Sprite {
    pub name: String,
    pub sheet_id: usize,
    pub rect: Rect,
}

impl Sprite {
    pub fn new(name: &str, sheet_id: usize, rect: Rect) -> Sprite {
        Sprite {
            name: String::from(name),
            sheet_id: sheet_id,
            rect: rect,
        }
    }
}
