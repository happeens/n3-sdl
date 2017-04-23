use sprite::Sprite;

#[derive(Serialize, Deserialize, Debug)]
struct AnimData {
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
    pub fn load(src: &str, sprites: &Vec<Sprite>) -> Vec<Animation> {
        let data: Vec<AnimData> = super::serde_json::from_str(src).unwrap();

        let mut result = Vec::new();
        for anim_data in data {
            let mut anim = Animation::new(&anim_data.name);
            for anim_frame_data in anim_data.frames {
                if let Some(index) = find_index(&anim_frame_data.name, &sprites) {
                    println!("found index: {}", index);
                    let frame = AnimFrame::new(index, 0.3);
                    anim.frames.push(frame);
                }
            }
            result.push(anim);
        }

        result
    }

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

fn find_index(name: &str, sprites: &Vec<Sprite>) -> Option<usize> {
    for (i, sprite) in sprites.iter().enumerate() {
        if sprite.name == name {
            return Some(i);
        }
    }

    None
}
