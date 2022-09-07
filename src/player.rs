use rltk::{Rltk, VirtualKeyCode};
use specs::prelude::*;
use std::cmp::{max, min};

use crate::State;
use crate::{xy_idx, TileType, MAPHEIGHT, MAPWIDTH};
use crate::{Player, Position};

pub fn try_move_player(dx: i32, dy: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let players = ecs.read_storage::<Player>();
    let map = ecs.fetch::<Vec<TileType>>();

    for (_, pos) in (&players, &mut positions).join() {
        let dest = xy_idx(pos.x + dx, pos.y + dy);
        if map[dest] != TileType::Wall {
            pos.x = min(MAPWIDTH as i32 - 1, max(0, pos.x + dx));
            pos.y = min(MAPHEIGHT as i32 - 1, max(0, pos.y + dy));
        }
    }
}

pub fn player_input(gs: &mut State, ctx: &mut Rltk) {
    if let Some(key) = ctx.key {
        match key {
            VirtualKeyCode::Left | VirtualKeyCode::Numpad4 | VirtualKeyCode::H => {
                try_move_player(-1, 0, &mut gs.ecs)
            }
            VirtualKeyCode::Right | VirtualKeyCode::Numpad6 | VirtualKeyCode::L => {
                try_move_player(1, 0, &mut gs.ecs)
            }
            VirtualKeyCode::Up | VirtualKeyCode::Numpad8 | VirtualKeyCode::K => {
                try_move_player(0, -1, &mut gs.ecs)
            }
            VirtualKeyCode::Down | VirtualKeyCode::Numpad2 | VirtualKeyCode::J => {
                try_move_player(0, 1, &mut gs.ecs)
            }
            _ => {}
        }
    }
}
