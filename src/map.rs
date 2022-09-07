use rltk::{Rltk, RGB};
use std::cmp::{max, min};

use crate::Rect;

// Map
pub const MAPWIDTH: usize = 80;
pub const MAPHEIGHT: usize = 50;

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
}

pub fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * MAPWIDTH) + x as usize
}

pub fn new_map_rooms_and_corridors() -> Vec<TileType> {
    let mut map = vec![TileType::Wall; MAPWIDTH * MAPHEIGHT];

    let room1 = Rect::new(20, 15, 10, 15);
    let room2 = Rect::new(35, 15, 10, 15);

    apply_room_to_map(&mut map, &room1);
    apply_room_to_map(&mut map, &room2);
    apply_horizontal_tunnel(&mut map, 25, 40, 23);

    map
}

fn apply_room_to_map(map: &mut [TileType], room: &Rect) {
    for y in room.y1 + 1..=room.y2 {
        for x in room.x1 + 1..=room.x2 {
            let idx = xy_idx(x, y);
            map[idx] = TileType::Floor;
        }
    }
}

fn apply_horizontal_tunnel(map: &mut [TileType], x1: i32, x2: i32, y: i32) {
    for x in min(x1, x2)..max(x1, x2) {
        let idx = xy_idx(x, y);
        if idx > 0 && idx < MAPWIDTH * MAPHEIGHT {
            map[idx] = TileType::Floor;
        }
    }
}

fn apply_vertical_tunnel(map: &mut [TileType], y1: i32, y2: i32, x: i32) {
    for y in min(y1, y2)..max(y1, y2) {
        let idx = xy_idx(x, y);
        if idx > 0 && idx < MAPWIDTH * MAPHEIGHT {
            map[idx] = TileType::Floor;
        }
    }
}

pub fn draw_map(map: &[TileType], ctx: &mut Rltk) {
    let mut x = 0;
    let mut y = 0;

    let grey = RGB::from_f32(0.5, 0.5, 0.5);
    let green = RGB::from_f32(0.0, 1.0, 0.0);

    for tile in map.iter() {
        match tile {
            TileType::Floor => {
                ctx.set(x, y, grey, rltk::BLACK, rltk::to_cp437('.'));
            }
            TileType::Wall => {
                ctx.set(x, y, green, rltk::BLACK, rltk::to_cp437('#'));
            }
        }

        x += 1;
        if x > MAPWIDTH as i32 - 1 {
            x = 0;
            y += 1;
        }
    }
}
