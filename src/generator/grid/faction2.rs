use std::mem;

use rand::{rngs::OsRng, Rng};

use rayon::prelude::*;
use smallvec::SmallVec;

use tile::Faction;

use level::GridLevel;

const LEHMER_MULT0: u128 = 0x9cec0193f9cb55c4acce1fe16e62b05f;
const LEHMER_MULT1: u128 = 0x82163e3e925f6e050dfa28d05eb25d83;
const LEHMER_MULT2: u128 = 0xdd72c15464faa3388de458ec1e58452b;
const LEHMER_MULT3: u128 = 0xb68e8e143dce210d5e40e89d3033fd65;
const LEHMER_MULT4: u128 = 0x12e15e35b500f16e2e714eb2b37916a5;

pub struct FactionGen {
    seed: u128,
}

impl FactionGen {
    pub fn new() -> Self {
        Self {
            seed: OsRng::new().unwrap().gen(),
        }
    }
    #[inline]
    pub fn generate(&mut self, level: &mut GridLevel<Faction>, buffer: &mut GridLevel<Faction>) {
        self.seed = tick(
            self.seed,
            level.get_width(),
            &level.tiles.data[..],
            &mut buffer.tiles.data[..],
        );
        mem::swap(level, buffer);
    }
}

struct CustomRng {
    lehmer1: u128,
    lehmer2: u128,
    lehmer3: u128,
    lehmer4: u128,
}

impl CustomRng {
    fn new(seed: u128) -> Self {
        Self {
            lehmer1: seed << 1 | 1,
            lehmer2: seed << 1 | 1,
            lehmer3: seed << 1 | 1,
            lehmer4: seed << 1 | 1,
        }
    }

    fn gen(&mut self) -> (u64, u64, u64, u64) {
        self.lehmer1 = self.lehmer1.wrapping_mul(LEHMER_MULT0);
        self.lehmer2 = self.lehmer2.wrapping_mul(LEHMER_MULT1);
        self.lehmer3 = self.lehmer3.wrapping_mul(LEHMER_MULT2);
        self.lehmer4 = self.lehmer4.wrapping_mul(LEHMER_MULT3);
        return (
            (self.lehmer1 >> 64) as u64,
            (self.lehmer2 >> 64) as u64,
            (self.lehmer3 >> 64) as u64,
            (self.lehmer4 >> 64) as u64,
        );
    }
}

fn select(deck: &[usize], n: u64) -> usize {
    deck[n as usize % deck.len()]
}

fn index(width: usize, x: i64, y: i64) -> usize {
    (x + y * width as i64) as usize
}

// TODO: Fix this
// TODO: Benchmark this
pub fn tick(seed: u128, width: usize, prev: &[Faction], next: &mut [Faction]) -> u128 {
    let height = prev.len() / width;

    next.par_chunks_mut(width)
        .enumerate()
        .for_each(|(y, chunk)| {
            let mut deck0 = SmallVec::<[_; 9]>::new();
            let mut deck1 = SmallVec::<[_; 9]>::new();
            let mut deck2 = SmallVec::<[_; 9]>::new();
            let mut deck3 = SmallVec::<[_; 9]>::new();
            let mut rng = CustomRng::new(seed ^ (y as u128 * 91));
            for x in (0..height).step_by(4) {
                let (r0, r1, r2, r3) = rng.gen();
                // x x x x x x
                // x o o o o x
                // x x x x x x
                for dy in -1..=1 {
                    for dx in -1..=4 {
                        let xx = x as i64 + dx;
                        let yy = y as i64 + dy;
                        let idx = index(width, xx, yy);
                        if let Some(Faction::Faction(f)) = prev.get(idx) {
                            if dx <= 1 {
                                deck0.push(*f);
                            }
                            if 0 <= dx && dx <= 2 {
                                deck1.push(*f);
                            }
                            if 1 <= dx && dx <= 3 {
                                deck2.push(*f);
                            }
                            if 2 <= dx {
                                deck3.push(*f);
                            }
                        }
                    }
                }

                if !deck0.is_empty() {
                    chunk[x] = Faction::Faction(select(&deck0[..], r0))
                }
                if !deck1.is_empty() {
                    chunk[x + 1] = Faction::Faction(select(&deck1[..], r1));
                }
                if !deck2.is_empty() {
                    chunk[x + 2] = Faction::Faction(select(&deck2[..], r2));
                }
                if !deck3.is_empty() {
                    chunk[x + 3] = Faction::Faction(select(&deck3[..], r3));
                }
                deck0.clear();
                deck1.clear();
                deck2.clear();
                deck3.clear();
            }
        });
    seed.wrapping_mul(LEHMER_MULT4)
}
