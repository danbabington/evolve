mod creature;
mod types;
mod world_iteration;

use crate::creature::Creature;
use crate::types::Position;
use crate::world_iteration::WorldIteration;

const WORLD_WIDTH: u32 = 10;
const WORLD_HEIGHT: u32 = 10;
const GENE_LENGTH: usize = 8;
const CREATURE_POPULATION: usize = 10;

fn main() {
    let mut world_iteration = WorldIteration::new();
    world_iteration.populate([[0; GENE_LENGTH]; CREATURE_POPULATION]);
    world_iteration.save_state_as_image("test.png");
}
