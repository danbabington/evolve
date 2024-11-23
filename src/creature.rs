use crate::types::Position;
use crate::GENE_LENGTH;
use image::Rgb;

#[derive(Debug, Copy, Clone)]
pub struct Creature {
    genes: [u32; GENE_LENGTH],
    position: Position,
    pub colour: Rgb<u8>,
}

fn calculate_colour(genes: [u32; GENE_LENGTH]) -> Rgb<u8> {
    let mod_24_bit = 1 << 24;
    let calculated = genes.iter().fold(0, |acc, &gene| (acc + gene) % mod_24_bit);
    let r = (calculated >> 16) as u8;
    let g = (calculated >> 8) as u8;
    let b = calculated as u8;
    Rgb([r, g, b])
}

impl Creature {
    pub fn new(genes: [u32; GENE_LENGTH], position: Position) -> Creature {
        let colour = calculate_colour(genes);
        Creature {
            genes,
            position,
            colour,
        }
    }
}
