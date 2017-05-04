use types::{Vec2, Size, Point};
use cgmath::EuclideanSpace;

const THRESHOLD: f64 = 0.1;

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

    pub fn as_vec(&self) -> Vec2 {
        Vec2::new(self.pos.x, self.pos.y)
    }

    pub fn update(&mut self, dt: f64) {
        let target = self.target + (self.screen.to_point() * -0.5).to_vec();
        let dist_x = target.x - self.pos.x;
        let dist_y = target.y - self.pos.y;
        let mut next_pos = self.pos + Vec2::new(dist_x, dist_y) * self.speed * dt;

        if (next_pos.x - self.pos.x).abs() <= THRESHOLD {
            next_pos.x = self.pos.x;
        }

        if (next_pos.y - self.pos.y).abs() <= THRESHOLD {
            next_pos.y = self.pos.y;
        }

        self.pos = next_pos;
    }
}
