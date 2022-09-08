use rltk::{console, Point};
use specs::prelude::*;

use crate::Monster;
use crate::Name;
use crate::Viewshed;

pub struct MonsterAiSystem {}

impl<'a> System<'a> for MonsterAiSystem {
    type SystemData = (
        ReadExpect<'a, Point>,
        ReadStorage<'a, Viewshed>,
        ReadStorage<'a, Monster>,
        ReadStorage<'a, Name>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (pos, viewshed, monster, name) = data;

        for (viewshed, _, name) in (&viewshed, &monster, &name).join() {
            if viewshed.visible_tiles.contains(&*pos) {
                console::log(&format!("{} shouts insults", name.name));
            }
        }
    }
}
