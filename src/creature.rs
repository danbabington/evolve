use crate::types::Position;
use crate::GENE_LENGTH;
use image::Rgb;

pub struct Creature {
    genes: [u32; GENE_LENGTH],
    position: Position,
    pub colour: Rgb<u8>,
}

impl Creature {
    pub fn new(genes: [u32; GENE_LENGTH], position: Position) -> Creature {
        Creature {
            genes,
            position,
            colour: Rgb([0, 0, 0]),
        }
    }
}
