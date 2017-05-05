use scene::Scene;
use tilemap::Tilemap;
use entity::{Player, PlayerData};
use types::{KeyAction, Point, Vec2, Direction};
use context::Context;

pub struct GameScene {
    map: Tilemap,
    player: Player,
}

impl GameScene {
    pub fn new(mut ctx: &mut Context) -> GameScene {
        let map = Tilemap::new(ctx);

        ctx.load_sheet("test");
        ctx.load_sheet("female0");

        let start_pos = Point::new(0.0, 0.0);
        let player_data: PlayerData = ::util::load_data("player-female0.json").unwrap();
        let player = Player::new(&player_data, start_pos, ctx.get_sprite_cache());

        GameScene {
            map: map,
            player: player,
        }
    }

    pub fn try_move_player(&mut self, v: Vec2) {
        // finding next player position for collision
        // let next_pos = self.player.next_pos(dt, v);
        // let new_x = self.player.next_pos(dt, Point::new(v.x(), 0.0));
        // let new_y = self.player.next_pos(dt, Point::new(0.0, v.y()));

        self.player.set_vel(v);
    }
}

impl Scene for GameScene {
    fn update(&mut self, ctx: &mut Context, dt: f32) {
        let mut move_intention = Vec2::new(0.0, 0.0);

        for key in ctx.held_keys() {
            match key {
                &KeyAction::Up => move_intention.y -= 1.0,
                &KeyAction::Down => move_intention.y += 1.0,
                &KeyAction::Left => move_intention.x -= 1.0,
                &KeyAction::Right => move_intention.x += 1.0,
            }
        }

        if move_intention.x != 0.0 || move_intention.y != 0.0 {
            use cgmath::prelude::*;
            move_intention = move_intention.normalize_to(1.0)
        }

        self.try_move_player(move_intention);

        match ctx.last_key() {
            Some(&KeyAction::Down) => self.player.set_facing(Direction::Down),
            Some(&KeyAction::Up) => self.player.set_facing(Direction::Up),
            Some(&KeyAction::Left) => self.player.set_facing(Direction::Left),
            Some(&KeyAction::Right) => self.player.set_facing(Direction::Right),
            _ => {}
        }

        let player_pos = self.player.get_pos();
        ctx.set_camera_target(player_pos);
        self.player.update(dt);
    }

    fn draw(&self, mut ctx: &mut Context, a: f32) {
        self.player.draw(ctx, a);
        self.map.draw(ctx);
    }
}
