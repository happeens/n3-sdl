use sdl2::EventPump as SdlEvents;
use sdl2::render::Renderer;
use sdl2::rect::Rect;
use sdl2::pixels::Color;

use sdl2::event::Event::*;
use sdl2::keyboard::Keycode::*;

use context::Context;
use scene::Scene;

use std::boxed::Box;
use std::cell::Cell;

use tilemap::Tilemap;

use types::{KeyAction, Size, Point, Vec2, Direction, RenderInfo};

use entity::{Player, PlayerData};

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
        let mut player = Player::new(&player_data, start_pos, ctx.get_sprite_cache());

        GameScene {
            map: map,
            player: player,
        }
    }

    pub fn try_move_player(&mut self, v: Vec2, dt: f64) {
        // finding next player position for collision
        // let next_pos = self.player.next_pos(dt, v);
        // let new_x = self.player.next_pos(dt, Point::new(v.x(), 0.0));
        // let new_y = self.player.next_pos(dt, Point::new(0.0, v.y()));

        self.player.set_vel(v);
    }
}

impl Scene for GameScene {
    fn update(&mut self, ctx: &mut Context, dt: f64) {
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

        self.try_move_player(move_intention, dt);

        match ctx.last_key() {
            Some(&KeyAction::Down) => self.player.set_facing(Direction::Down),
            Some(&KeyAction::Up) => self.player.set_facing(Direction::Up),
            Some(&KeyAction::Left) => self.player.set_facing(Direction::Left),
            Some(&KeyAction::Right) => self.player.set_facing(Direction::Right),
            _ => {}
        }

        let player_pos = self.player.get_pos();
        ctx.set_camera_target(player_pos);
        self.player.update(ctx, dt);
    }

    fn draw(&self, mut ctx: &mut Context) {
        self.player.draw(ctx);
        self.map.draw(ctx);
    }
}
