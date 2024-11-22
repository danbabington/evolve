mod creature;
mod data_types;
mod world;

use crate::creature::Creature;
use crate::data_types::Position;
use crate::world::World;

fn main() {
    let mut world = World::new(10, 10);
    let creature_1 = Creature::new(Position { x: 0, y: 0 });
    world.populate();
    // world.cells[0] = Some(creature_1);
    println!("Hello, world!");
}
