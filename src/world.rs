use crate::creature::Creature;
use crate::data_types::Position;

pub struct World {
    width: u16,
    height: u16,
    pub cells: Vec<Option<Creature>>,
}

impl World {
    pub fn new(width: u16, height: u16) -> World {
        let mut empty_cells = Vec::new();
        empty_cells.resize_with((width * height).into(), Default::default);
        World {
            width,
            height,
            cells: empty_cells,
        }
    }

    pub fn populate(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let index = self.position_to_index(Position { x, y });
                let position = Position { x, y };
                let creature = Creature::new(position);
                self.cells[index] = Some(creature);
            }
        }
    }

    fn position_to_index(&self, position: Position) -> usize {
        (position.y * self.width + position.x) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_empty_world_with_unique_cells() {
        let mut world = World::new(10, 10);
        assert_eq!(world.width, 10);
        assert_eq!(world.height, 10);
        assert_eq!(world.cells.len(), 100);
        let creature_1 = Creature::new(Position { x: 0, y: 0 });
        let creature_2 = Creature::new(Position { x: 1, y: 1 });
        world.cells[0] = Some(creature_1);
        world.cells[1] = Some(creature_2);
    }
}
