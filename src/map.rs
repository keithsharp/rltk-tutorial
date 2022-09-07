use rltk::{RandomNumberGenerator, Rltk, RGB};
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

pub fn new_map_rooms_and_corridors() -> (Vec<Rect>, Vec<TileType>) {
    let mut map = vec![TileType::Wall; MAPWIDTH * MAPHEIGHT];

    let mut rooms = Vec::new();
    const MAX_ROOMS: i32 = 30;
    const MIN_SIZE: i32 = 6;
    const MAX_SIZE: i32 = 10;

    let mut rng = RandomNumberGenerator::new();

    for _ in 0..MAX_ROOMS {
        let w = rng.range(MIN_SIZE, MAX_SIZE);
        let h = rng.range(MIN_SIZE, MAX_SIZE);
        let x = rng.roll_dice(1, MAPWIDTH as i32 - w - 1) - 1;
        let y = rng.roll_dice(1, MAPHEIGHT as i32 - h - 1) - 1;
        let room = Rect::new(x, y, w, h);

        let mut ok = true;
        for other in rooms.iter() {
            if room.intersect(other) {
                ok = false
            }
        }
        if ok {
            apply_room_to_map(&mut map, &room);

            if !rooms.is_empty() {
                let (new_x, new_y) = room.center();
                let (prev_x, prev_y) = rooms[rooms.len() - 1].center();
                match rng.range(0, 2) {
                    1 => {
                        apply_horizontal_tunnel(&mut map, prev_x, new_x, prev_y);
                        apply_vertical_tunnel(&mut map, prev_y, new_y, new_x);
                    }
                    _ => {
                        apply_vertical_tunnel(&mut map, prev_y, new_y, prev_x);
                        apply_horizontal_tunnel(&mut map, prev_x, new_x, new_y);
                    }
                }
            }

            rooms.push(room);
        }
    }

    (rooms, map)
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
    for x in min(x1, x2)..=max(x1, x2) {
        let idx = xy_idx(x, y);
        if idx > 0 && idx < MAPWIDTH * MAPHEIGHT {
            map[idx] = TileType::Floor;
        }
    }
}

fn apply_vertical_tunnel(map: &mut [TileType], y1: i32, y2: i32, x: i32) {
    for y in min(y1, y2)..=max(y1, y2) {
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
