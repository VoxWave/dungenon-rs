use rand::os::OsRng;
use rand::XorShiftRng;
use rand::Rand;
use rand::Rng;

use tile::Faction;

pub struct FactionGen {
    rand: XorShiftRng,
    faction_amount: usize,
}

impl FactionGen {
    pub fn new(faction_amount: usize) -> FactionGen {
        FactionGen {
            rand: XorShiftRng::rand(&mut OsRng::new().unwrap()),
            faction_amount: faction_amount,
        }
    }

    pub fn generate(&mut self, level: &mut Level<Faction>) {
        let delta = Vec::new();
        for x in 0..level.get_width() {
            for y in 0..level.get_height() {
                match level.get_mut_tile(x, y) {
                    Ok(tile) => {
                        let mut deck = Vec::new();
                        match *tile {
                            Faction::Faction(_) => {
                                deck.push(tile.clone());
                            },
                            Faction::Void => continue,
                            _ => {},
                        }
                        get_faction_neighbours(&mut deck, &mut level);
                        self.rand.shuffle(&mut deck);
                        match deck.pop() {
                            Some(f) => {
                                delta.push(((x,y), f));
                            },
                            None => continue,
                        }
                    },
                    Err(Error::IndexOutOfBounds) => {
                        panic!("Generate method indexed out of bounds while simulating a step. This should never happen unless the programmer is not very bright.");
                    }
                }
            }
        }
        Self::applyChanges(level, delta);
    }

    fn applyChanges(level: &mut Level<Faction>, delta: Vec<((usize, usize), Faction)>) {
        for ((x, y), f) in delta {
            if let Ok(tile) = level.get_mut_tile(x, y) {
                *tile = f;
            }
        }
    }
}
