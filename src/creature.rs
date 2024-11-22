use crate::types::Position;
use crate::GENE_LENGTH;
use image::Rgb;

pub struct Creature {
    genes: [u32; GENE_LENGTH],
    position: Position,
}

impl Creature {
    pub fn new(genes: [u32; GENE_LENGTH], position: Position) -> Creature {
        Creature { genes, position }
    }

    pub fn get_colour(&self) -> Rgb<u8> {
        let red = self.genes[0] as u8;
        let green = self.genes[1] as u8;
        let blue = self.genes[2] as u8;
        Rgb([red, green, blue])
    }
}
