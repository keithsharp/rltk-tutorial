use rltk::{GameState, Rltk};
use specs::prelude::*;

use crate::player_input;
use crate::Map;
use crate::MonsterAiSystem;
use crate::VisibilitySystem;
use crate::{Position, Renderable};

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum RunState {
    Paused,
    Running,
}

pub struct State {
    pub ecs: World,
    pub runstate: RunState,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        if self.runstate == RunState::Running {
            self.run_systems();
            self.runstate = RunState::Paused
        } else {
            self.runstate = player_input(self, ctx);
        }

        ctx.cls();

        let map = self.ecs.fetch::<Map>();
        map.draw(ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        let map = self.ecs.fetch::<Map>();

        for (pos, render) in (&positions, &renderables).join() {
            let idx = map.xy_to_idx(pos.x, pos.y);
            if map.visible_tiles[idx] {
                ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
            }
        }
    }
}

impl State {
    fn run_systems(&mut self) {
        let mut vis = VisibilitySystem {};
        vis.run_now(&self.ecs);

        let mut mob = MonsterAiSystem {};
        mob.run_now(&self.ecs);

        self.ecs.maintain();
    }
}
