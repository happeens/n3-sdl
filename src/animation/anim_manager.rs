use sprite::SpriteManager;
use animation::{Animation, AnimFrame, AnimData};

pub struct AnimManager {
    anims: Vec<Animation>,
    timer: f32,
    anim_running: bool,
    current_anim: usize,
    current_frame: usize
}

impl AnimManager {
    pub fn new(data: &Vec<AnimData>, sprites: &SpriteManager) -> AnimManager {
        let mut anims = Vec::new();
        for anim_data in data.iter() {
            let mut anim = Animation::new(&anim_data.name);
            for anim_frame_data in anim_data.frames.iter() {
                if let Some(index) = sprites.find_index(&anim_frame_data.name) {
                    let frame = AnimFrame::new(index, 0.2);
                    anim.frames.push(frame);
                }
            }
            anims.push(anim);
        }

        AnimManager {
            anims: anims,
            timer: 0.0,
            anim_running: false,
            current_anim: 0,
            current_frame: 0
        }
    }

    pub fn current(&self) -> usize {
        self.anims[self.current_anim].frames[self.current_frame].index
    }

    pub fn update(&mut self, dt: f32) {
        if !self.anim_running {
            return;
        }

        self.advance_anim(dt);
    }

    pub fn run(&mut self, name: &str) {
        if self.anim_running && self.anims[self.current_anim].name == name {
            return;
        }

        if let Some(index) = self.find_anim(name) {
            self.anim_running = true;
            self.timer = 0.0;
            self.current_anim = index;
            self.current_frame = 0;
        } else {
            println!("animation not found: {}", name);
        }
    }

    pub fn stop_anim(&mut self) {
        self.anim_running = false;
    }

    pub fn anim_running(&self) -> bool {
        self.anim_running
    }

    fn advance_anim(&mut self, dt: f32) {
        self.timer += dt;

        if self.timer > self.get_delay(self.current_anim, self.current_frame) {
            self.next_frame();
        }
    }

    fn next_frame(&mut self) {
        self.timer = 0.0;
        self.current_frame += 1;

        if self.current_frame >= self.anims[self.current_anim].frames.len() {
            self.current_frame = 0;
        }
    }

    fn get_delay(&self, anim: usize, frame: usize) -> f32 {
        self.anims[anim].frames[frame].delay
    }

    fn find_anim(&self, name: &str) -> Option<usize> {
        for (i, anim) in self.anims.iter().enumerate() {
            if anim.name == name {
                return Some(i);
            }
        }
        
        None
    }
}