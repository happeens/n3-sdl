use types::{Point, Size};

const THRESHOLD: f64 = 0.3;

pub struct Camera {
    pos: Point,
    target: Point,
    speed: f64,
    screen: Size,
}

impl Camera {
    pub fn new(target: Point, screen: Size, speed: f64) -> Camera {
        Camera {
            pos: Point::new(0.0, 0.0),
            target: target,
            screen: screen,
            speed: speed
        }
    }

    pub fn set_target(&mut self, target: Point) {
        self.target = target;
    } 

    pub fn get_pos(&self) -> Point {
        self.pos
    }

    pub fn update(&mut self, dt: f64) {
        let target = self.target - self.screen.to_point() * 0.5;
        let distance = target - self.pos;
        let mut next_pos = self.pos + distance * self.speed * dt;

        if (next_pos.x - self.pos.x).abs() <= THRESHOLD {
            next_pos.x = self.pos.x;
        }

        if (next_pos.y - self.pos.y).abs() <= THRESHOLD {
            next_pos.y = self.pos.y;
        }

        self.pos = next_pos;
    }
}
