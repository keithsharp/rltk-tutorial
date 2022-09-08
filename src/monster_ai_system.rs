use rltk::{console, Point};
use specs::prelude::*;

use crate::Monster;
use crate::Viewshed;

pub struct MonsterAiSystem {}

impl<'a> System<'a> for MonsterAiSystem {
    type SystemData = (
        ReadExpect<'a, Point>,
        ReadStorage<'a, Viewshed>,
        ReadStorage<'a, Monster>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (pos, viewshed, monster) = data;

        for (viewshed, _) in (&viewshed, &monster).join() {
            if viewshed.visible_tiles.contains(&*pos) {
                console::log("Monster shouts insults");
            }
        }
    }
}
