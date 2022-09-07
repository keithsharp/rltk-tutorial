use rltk::{Rltk, RGB};

// Map
pub const MAPWIDTH: usize = 80;
pub const MAPHEIGHT: usize = 50;

#[derive(PartialEq, Copy, Clone)]
pub enum TileType  {
    Wall,
    Floor,
}

pub fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * MAPWIDTH) + x as usize
}

pub fn new_map() -> Vec<TileType> {
    let mut map = vec![TileType::Floor; MAPWIDTH * MAPHEIGHT];
    
    for x in 0..MAPWIDTH as i32 {
        map[xy_idx(x, 0)] = TileType::Wall;
        map[xy_idx(x, MAPHEIGHT as i32 - 1)] = TileType::Wall;
    }
    for y in 0..MAPHEIGHT as i32 {
        map[xy_idx(0, y)] = TileType::Wall;
        map[xy_idx(MAPWIDTH as i32 - 1, y)] = TileType::Wall;
    }

    let mut rng = rltk::RandomNumberGenerator::new();
    for _ in 0..400 {
        let x = rng.roll_dice(1, MAPWIDTH as i32 - 1);
        let y = rng.roll_dice(1, MAPHEIGHT as i32 - 1);

        let idx = xy_idx(x, y);
        if idx != xy_idx(40, 25) {
            map[idx] = TileType::Wall;
        }
    }

    map
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
            },
            TileType::Wall => {
                ctx.set(x, y, green, rltk::BLACK, rltk::to_cp437('#'));
            },
        }

        x += 1;
        if x > MAPWIDTH as i32 - 1 {
            x = 0;
            y += 1;
        }
    }
}
