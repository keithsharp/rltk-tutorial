use rltk::{Point, RGB};
use specs::prelude::*;

mod components;
use components::*;

mod game_state;
use game_state::*;

mod map;
use map::*;

mod monster_ai_system;
use monster_ai_system::*;

mod player;
use player::*;

mod rect;
use rect::*;

mod visibility_system;
use visibility_system::*;

const PLAYER_VIEW_RANGE: i32 = 8;
const MONSTER_VIEW_RANGE: i32 = 8;

// Main Function
fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;

    let mut gs = State {
        ecs: World::new(),
        runstate: RunState::Running,
    };
    gs.ecs.register::<Monster>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Viewshed>();

    let map = Map::new_map_rooms_and_corridors();
    let (player_x, player_y) = map.rooms[0].center();

    let mut rng = rltk::RandomNumberGenerator::new();
    for room in map.rooms.iter().skip(1) {
        let (x, y) = room.center();
        let glyph = match rng.roll_dice(1, 2) {
            1 => rltk::to_cp437('g'),
            _ => rltk::to_cp437('o'),
        };

        gs.ecs
            .create_entity()
            .with(Position { x, y })
            .with(Renderable {
                glyph,
                fg: RGB::named(rltk::RED),
                bg: RGB::named(rltk::BLACK),
            })
            .with(Viewshed {
                visible_tiles: Vec::new(),
                range: MONSTER_VIEW_RANGE,
                dirty: true,
            })
            .with(Monster {})
            .build();
    }

    gs.ecs
        .create_entity()
        .with(Position {
            x: player_x,
            y: player_y,
        })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player {})
        .with(Viewshed {
            visible_tiles: Vec::new(),
            range: PLAYER_VIEW_RANGE,
            dirty: true,
        })
        .build();
    gs.ecs.insert(Point::new(player_x, player_y));

    gs.ecs.insert(map);

    rltk::main_loop(context, gs)
}
