use rltk::{Algorithm2D, BaseMap, Point, RandomNumberGenerator, Rltk, RGB};
use std::cmp::{max, min};

use crate::Rect;

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
    pub revealed_tiles: Vec<bool>,
    pub visible_tiles: Vec<bool>,
}

impl Map {
    pub fn new_map_rooms_and_corridors() -> Map {
        let mut map = Map {
            tiles: vec![TileType::Wall; (MAP_WIDTH * MAP_HEIGHT) as usize],
            rooms: Vec::new(),
            width: MAP_WIDTH,
            height: MAP_HEIGHT,
            revealed_tiles: vec![false; (MAP_WIDTH * MAP_HEIGHT) as usize],
            visible_tiles: vec![false; (MAP_WIDTH * MAP_HEIGHT) as usize],
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

    pub fn xy_to_idx(&self, x: i32, y: i32) -> usize {
        (y as usize * self.width as usize) + x as usize
    }

    pub fn idx_to_xy(&self, idx: usize) -> (i32, i32) {
        let x = idx as i32 % self.width;
        let y = idx as i32 / self.width;
        (x, y)
    }

    fn apply_room_to_map(&mut self, room: &Rect) {
        for y in room.y1 + 1..=room.y2 {
            for x in room.x1 + 1..=room.x2 {
                let idx = self.xy_to_idx(x, y);
                self.tiles[idx] = TileType::Floor;
            }
        }
    }

    fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        for x in min(x1, x2)..=max(x1, x2) {
            let idx = self.xy_to_idx(x, y);
            if idx > 0 && idx < (self.width * self.height) as usize {
                self.tiles[idx] = TileType::Floor;
            }
        }
    }

    fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        for y in min(y1, y2)..=max(y1, y2) {
            let idx = self.xy_to_idx(x, y);
            if idx > 0 && idx < (self.width * self.height) as usize {
                self.tiles[idx] = TileType::Floor;
            }
        }
    }

    pub fn draw(&self, ctx: &mut Rltk) {
        let grey = RGB::from_f32(0.5, 0.5, 0.5);
        let green = RGB::from_f32(0.0, 1.0, 0.0);

        let mut x = 0;
        let mut y = 0;

        for (idx, tile) in self.tiles.iter().enumerate() {
            if self.revealed_tiles[idx] {
                let (glyph, mut fg) = match tile {
                    TileType::Floor => (rltk::to_cp437('.'), grey),
                    TileType::Wall => (rltk::to_cp437('#'), green),
                };
                if !self.visible_tiles[idx] {
                    fg = fg.to_greyscale();
                }
                ctx.set(x, y, fg, rltk::BLACK, glyph);
            }

            x += 1;
            if x > self.width as i32 - 1 {
                x = 0;
                y += 1;
            }
        }
    }

    fn is_valid_exit(&self, x: i32, y: i32) -> bool {
        if x < 1 || x > self.width - 1 || y < 1 || y > self.height - 1 {
            return false;
        }
        let idx = self.xy_to_idx(x, y);
        self.tiles[idx] != TileType::Wall
    }
}

impl BaseMap for Map {
    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx] == TileType::Wall
    }

    fn get_available_exits(&self, idx: usize) -> rltk::SmallVec<[(usize, f32); 10]> {
        let mut exits = rltk::SmallVec::new();
        let (x, y) = self.idx_to_xy(idx);
        let w = self.width as usize;

        if self.is_valid_exit(x - 1, y) {
            exits.push((idx - 1, 1.0))
        };
        if self.is_valid_exit(x + 1, y) {
            exits.push((idx + 1, 1.0))
        };
        if self.is_valid_exit(x, y - 1) {
            exits.push((idx - w, 1.0))
        };
        if self.is_valid_exit(x, y + 1) {
            exits.push((idx + w, 1.0))
        };

        exits
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        let w = self.width as usize;
        let p1 = Point::new(idx1 % w, idx1 / w);
        let p2 = Point::new(idx2 % w, idx2 / w);
        rltk::DistanceAlg::Pythagoras.distance2d(p1, p2)
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> rltk::Point {
        Point::new(self.width, self.height)
    }
}
