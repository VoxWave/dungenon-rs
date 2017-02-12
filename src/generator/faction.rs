use rand::os::OsRng;
use rand::XorShiftRng;
use rand::Rand;
use rand::Rng;

use tile::Faction;

use util::{Error, Direction};

use level::{Level, add_isize_to_usize};

pub struct FactionGen {
    rand: XorShiftRng,
}

impl FactionGen {
    pub fn new() -> FactionGen {
        FactionGen {
            rand: XorShiftRng::rand(&mut OsRng::new().unwrap()),
        }
    }

    pub fn generate(&mut self, level: &mut Level<Faction>) {
        let mut delta = Vec::new();
        for x in 0..level.get_width() {
            for y in 0..level.get_height() {
                let mut deck = Vec::new();
                match level.get_mut_tile(x, y) {
                    Ok(tile) => {
                        match *tile {
                            Faction::Faction(_) => {
                                deck.push(tile.clone());
                            },
                            Faction::Void => continue,
                            _ => {},
                        }

                    },
                    Err(Error::IndexOutOfBounds) => {
                        panic!("Generate method indexed out of bounds while simulating a step. This should never happen unless the programmer is not very bright.");
                    }
                }
                Self::get_faction_neighbours(x, y, &mut deck, level);
                match self.rand.choose(&deck) {
                    Some(f) => {
                        delta.push(((x,y), f.clone()));
                    },
                    None => continue,
                }
            }
        }
        Self::apply_changes(level, delta);
    }

    fn get_faction_neighbours(x: usize, y: usize, deck: &mut Vec<Faction>, level: &mut Level<Faction>) {
        for d in Direction::get_dirs() {
            let (ix, iy) = d.get_tuple();
            let coord = match (add_isize_to_usize(ix, x), add_isize_to_usize(iy, y)) {
                (Some(x), Some(y)) => (x,y),
                _ => continue,
            };
            match level.get_tile_with_tuple(coord) {
                Ok(f @ &Faction::Faction(_)) => deck.push(f.clone()),
                _ => {},
            }
        }
    }

    fn apply_changes(level: &mut Level<Faction>, delta: Vec<((usize, usize), Faction)>) {
        for ((x, y), f) in delta {
            if let Ok(tile) = level.get_mut_tile(x, y) {
                *tile = f;
            }
        }
    }
}
