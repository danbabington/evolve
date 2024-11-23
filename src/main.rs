mod creature;
mod types;
mod world;

use crate::creature::Creature;
use crate::types::Position;
use crate::world::World;

const WORLD_WIDTH: u32 = 50;
const WORLD_HEIGHT: u32 = 50;
const GENE_LENGTH: usize = 8;
const CREATURE_POPULATION: usize = 100;
const IMAGE_SCALE: u32 = 10;

fn main() {
    let mut world_iteration = World::new();
    world_iteration.populate(random_initial_genes());
    world_iteration.save_state_as_image("test.png");
}

fn random_initial_genes() -> [[u32; GENE_LENGTH]; CREATURE_POPULATION] {
    std::array::from_fn(|_| std::array::from_fn(|_| rand::random::<u32>()))
}
