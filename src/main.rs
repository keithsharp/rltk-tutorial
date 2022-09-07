use rltk::RGB;
use specs::prelude::*;

mod components;
use components::*;

mod game_state;
use game_state::*;

mod map;
use map::*;

mod player;
use player::*;

mod rect;
use rect::*;

mod visibility_system;
use visibility_system::*;

const PLAYER_VIEW_RANGE: i32 = 8;

// Main Function
fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;

    let mut gs = State { ecs: World::new() };
    gs.ecs.register::<Player>();
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Viewshed>();

    let map = Map::new_map_rooms_and_corridors();
    let (player_x, player_y) = map.rooms[0].center();

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
        })
        .build();

    gs.ecs.insert(map);

    rltk::main_loop(context, gs)
}
