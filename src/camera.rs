use types::{Vec2, Size, Point};
use cgmath::EuclideanSpace;

const THRESHOLD: f32 = 0.8;

pub struct Camera {
    pos: Point,
    target: Point,
    speed: f32,
    screen: Size,
}

impl Camera {
    pub fn new(target: Point, screen: Size, speed: f32) -> Camera {
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

    pub fn next_vec(&self, a: f32) -> Vec2 {
        self.next_pos(a).to_vec() * -1.0
    }

    fn next_pos(&self, a: f32) -> Point {
        let target = self.target + (self.screen.to_point() * -0.5).to_vec();
        let dist_x = target.x - self.pos.x;
        let dist_y = target.y - self.pos.y;
        let mut result = self.pos + Vec2::new(dist_x, dist_y) * self.speed * a;

        if (result.x - self.pos.x).abs() <= THRESHOLD {
            result.x = self.pos.x;
        }

        if (result.y - self.pos.y).abs() <= THRESHOLD {
            result.y = self.pos.y;
        }

        result
    }

    pub fn update(&mut self, dt: f32) {
        self.pos = self.next_pos(dt);
    }
}
