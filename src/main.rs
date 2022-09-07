use rltk::{GameState, Rltk, VirtualKeyCode, RGB, Tile};
use specs::prelude::*;
use specs_derive::Component;
use std::cmp::{max, min};

// Map
const MAPWIDTH: usize = 80;
const MAPHEIGHT: usize = 50;

#[derive(PartialEq, Copy, Clone)]
enum TileType  {
    Wall,
    Floor,
}

fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * MAPWIDTH) + x as usize
}

fn new_map() -> Vec<TileType> {
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

fn draw_map(map: &[TileType], ctx: &mut Rltk) {
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

// Components
#[derive(Component)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Renderable {
    glyph: rltk::FontCharType,
    fg: rltk::RGB,
    bg: rltk::RGB,
}

#[derive(Component)]
struct LeftMover {}

#[derive(Component)]
struct Player {}

// Systems
struct LeftWalker {}
impl<'a> System<'a> for LeftWalker {
    type SystemData = (ReadStorage<'a, LeftMover>, WriteStorage<'a, Position>);

    fn run(&mut self, data: Self::SystemData) {
        let (lefty, mut pos) = data;
        for (_, mut pos) in (&lefty, &mut pos).join() {
            pos.x -= 1;
            if pos.x < 0 {
                pos.x = MAPWIDTH as i32 - 1;
            }
        }
    }
}

// Helper Functions
fn try_move_player(dx: i32, dy: i32, ecs: &mut World) {
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

fn player_input(gs: &mut State, ctx: &mut Rltk) {
    if let Some(key) = ctx.key {
        match key {
            VirtualKeyCode::Left => try_move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::Right => try_move_player(1, 0, &mut gs.ecs),
            VirtualKeyCode::Up => try_move_player(0, -1, &mut gs.ecs),
            VirtualKeyCode::Down => try_move_player(0, 1, &mut gs.ecs),
            _ => {}
        }
    }
}

// Game State
struct State {
    ecs: World,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        player_input(self, ctx);
        self.run_systems();

        ctx.cls();

        let map = self.ecs.fetch::<Vec<TileType>>();
        draw_map(&map, ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}

impl State {
    fn run_systems(&mut self) {
        let mut lw = LeftWalker {};
        lw.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

// Main Function
fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;

    let mut gs = State { ecs: World::new() };
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<LeftMover>();
    gs.ecs.register::<Player>();

    gs.ecs
        .create_entity()
        .with(Position { x: 40, y: 25 })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player {})
        .build();

    for i in 0..10 {
        gs.ecs
            .create_entity()
            .with(Position { x: i * 7, y: 20 })
            .with(Renderable {
                glyph: rltk::to_cp437('@'),
                fg: RGB::named(rltk::RED),
                bg: RGB::named(rltk::BLACK),
            })
            .with(LeftMover {})
            .build();
    }

    gs.ecs.insert(new_map());

    rltk::main_loop(context, gs)
}
