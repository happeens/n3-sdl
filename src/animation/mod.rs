mod anim_manager;
pub type AnimManager = anim_manager::AnimManager;

#[derive(Serialize, Deserialize, Debug)]
pub struct AnimData {
    name: String,
    frames: Vec<AnimFrameData>
}

#[derive(Serialize, Deserialize, Debug)]
struct AnimFrameData {
    name: String
}

pub struct Animation {
    pub name: String,
    pub frames: Vec<AnimFrame>
}

pub struct AnimFrame {
    pub index: usize,
    pub delay: f64
}

impl Animation {
    pub fn new(name: &str) -> Animation {
        Animation {
            name: String::from(name),
            frames: Vec::new()
        }
    }
}

impl AnimFrame {
    pub fn new(index: usize, delay: f64) -> AnimFrame {
        AnimFrame {
            index: index,
            delay: delay
        }
    }
}
