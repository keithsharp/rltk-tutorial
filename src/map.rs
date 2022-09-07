use rltk::{Algorithm2D, BaseMap, Point, RandomNumberGenerator, Rltk, RGB};
use specs::prelude::*;
use std::cmp::{max, min};

use crate::Rect;
use crate::{Player, Viewshed};

const MAP_WIDTH: i32 = 80;
const MAP_HEIGHT: i32 = 50;

const MAX_ROOMS: i32 = 30;
const MIN_SIZE: i32 = 6;
const MAX_SIZE: i32 = 10;

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct Map {
    pub tiles: Vec<TileType>,
    pub rooms: Vec<Rect>,
    pub width: i32,
    pub height: i32,
}

impl Map {
    pub fn new_map_rooms_and_corridors() -> Map {
        let mut map = Map {
            tiles: vec![TileType::Wall; (MAP_WIDTH * MAP_HEIGHT) as usize],
            rooms: Vec::new(),
            width: MAP_WIDTH,
            height: MAP_HEIGHT,
        };

        let mut rng = RandomNumberGenerator::new();

        for _ in 0..MAX_ROOMS {
            let w = rng.range(MIN_SIZE, MAX_SIZE);
            let h = rng.range(MIN_SIZE, MAX_SIZE);
            let x = rng.roll_dice(1, map.width - w - 1) - 1;
            let y = rng.roll_dice(1, map.height - h - 1) - 1;
            let room = Rect::new(x, y, w, h);

            let mut ok = true;
            for other in map.rooms.iter() {
                if room.intersect(other) {
                    ok = false
                }
            }
            if ok {
                map.apply_room_to_map(&room);

                if !map.rooms.is_empty() {
                    let (new_x, new_y) = room.center();
                    let (prev_x, prev_y) = map.rooms[map.rooms.len() - 1].center();
                    match rng.range(0, 2) {
                        1 => {
                            map.apply_horizontal_tunnel(prev_x, new_x, prev_y);
                            map.apply_vertical_tunnel(prev_y, new_y, new_x);
                        }
                        _ => {
                            map.apply_vertical_tunnel(prev_y, new_y, prev_x);
                            map.apply_horizontal_tunnel(prev_x, new_x, new_y);
                        }
                    }
                }

                map.rooms.push(room);
            }
        }

        map
    }

    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y as usize * self.width as usize) + x as usize
    }

    fn apply_room_to_map(&mut self, room: &Rect) {
        for y in room.y1 + 1..=room.y2 {
            for x in room.x1 + 1..=room.x2 {
                let idx = self.xy_idx(x, y);
                self.tiles[idx] = TileType::Floor;
            }
        }
    }

    fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        for x in min(x1, x2)..=max(x1, x2) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx < (self.width * self.height) as usize {
                self.tiles[idx] = TileType::Floor;
            }
        }
    }

    fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        for y in min(y1, y2)..=max(y1, y2) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx < (self.width * self.height) as usize {
                self.tiles[idx] = TileType::Floor;
            }
        }
    }

    pub fn draw(&self, ecs: &World, ctx: &mut Rltk) {
        let mut viewsheds = ecs.write_storage::<Viewshed>();
        let mut players = ecs.write_storage::<Player>();

        let grey = RGB::from_f32(0.5, 0.5, 0.5);
        let green = RGB::from_f32(0.0, 1.0, 0.0);

        for (_, viewshed) in (&mut players, &mut viewsheds).join() {
            let mut x = 0;
            let mut y = 0;

            for tile in self.tiles.iter() {
                let point = Point::new(x, y);
                if viewshed.visible_tiles.contains(&point) {
                    match tile {
                        TileType::Floor => {
                            ctx.set(x, y, grey, rltk::BLACK, rltk::to_cp437('.'));
                        }
                        TileType::Wall => {
                            ctx.set(x, y, green, rltk::BLACK, rltk::to_cp437('#'));
                        }
                    }
                }

                x += 1;
                if x > self.width as i32 - 1 {
                    x = 0;
                    y += 1;
                }
            }
        }
    }
}

impl BaseMap for Map {
    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx] == TileType::Wall
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> rltk::Point {
        Point::new(self.width, self.height)
    }
}
