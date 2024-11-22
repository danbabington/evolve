extern crate image;
extern crate rand;

use image::{ImageBuffer, Rgb};

use crate::creature::Creature;
use crate::types::Position;
use crate::CREATURE_POPULATION;
use crate::GENE_LENGTH;
use crate::WORLD_HEIGHT;
use crate::WORLD_WIDTH;

const WORLD_SIZE: usize = (WORLD_HEIGHT * WORLD_WIDTH) as usize;
const BACKGROUND_COLOUR: Rgb<u8> = Rgb([255, 255, 255]);

pub struct WorldIteration {
    occupied_cells: [bool; WORLD_SIZE],
    cell_colour: [Rgb<u8>; WORLD_SIZE],
    creatures: [Option<Creature>; CREATURE_POPULATION],
}

fn position_to_index(position: &Position) -> usize {
    (position.y * WORLD_WIDTH + position.x) as usize
}

impl WorldIteration {
    pub fn new() -> WorldIteration {
        WorldIteration {
            occupied_cells: [false; WORLD_SIZE],
            cell_colour: [BACKGROUND_COLOUR; WORLD_SIZE],
            creatures: std::array::from_fn(|_| None),
        }
    }

    pub fn populate(&mut self, genes: [[u32; GENE_LENGTH]; CREATURE_POPULATION]) {
        for i in 0..CREATURE_POPULATION {
            let position = self.random_available_position();
            let cell_index = position_to_index(&position);
            self.occupied_cells[cell_index] = true;
            let new_creature = Creature::new(genes[i], position);
            self.cell_colour[cell_index] = new_creature.colour;
            self.creatures[i] = Some(new_creature);
        }
    }

    pub fn random_available_position(&self) -> Position {
        loop {
            let position = Position {
                x: rand::random::<u32>() % WORLD_WIDTH,
                y: rand::random::<u32>() % WORLD_HEIGHT,
            };
            if !self.occupied_cells[position_to_index(&position)] {
                return position;
            }
        }
    }

    pub fn save_state_as_image(&self, filename: &str) {
        let mut img = ImageBuffer::new(WORLD_WIDTH, WORLD_HEIGHT);
        for y in 0..WORLD_HEIGHT {
            for x in 0..WORLD_WIDTH {
                let position = Position { x, y };
                let cell_index = position_to_index(&position);
                img.put_pixel(x, y, self.cell_colour[cell_index]);
            }
        }
        img.save(filename).unwrap();
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn create_empty_world_with_unique_cells() {
//         let mut world = WorldIteration::new(10, 10);
//         assert_eq!(world.width, 10);
//         assert_eq!(world.height, 10);
//         assert_eq!(world.cells.len(), 100);
//         world.populate();
//     }
// }
