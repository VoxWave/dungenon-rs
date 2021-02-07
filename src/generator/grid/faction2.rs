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
            seed: OsRng::new().unwrap().gen::<u128>() << 1 | 1,
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

struct LehmerRng {
    lehmer: u128,
    mult: u128,
}

impl LehmerRng {
    fn new(seed: u128, mult: u128) -> Self {
        Self {
            lehmer: seed << 1 | 1,
            mult,
        }
    }

    fn gen(&mut self) -> u64 {
        self.lehmer = self.lehmer.wrapping_mul(self.mult);
        return (self.lehmer >> 64) as u64;
    }
}

fn select(deck: &[usize], n: u64) -> usize {
    deck[n as usize % deck.len()]
}

fn index(width: usize, x: i64, y: i64) -> usize {
    (x + y * width as i64) as usize
}

fn mix(a: u128, b: usize, mult: u128) -> u128 {
    a ^ ((b << 1 | 1) as u128).wrapping_mul(mult)
}

pub fn tick(seed: u128, width: usize, prev: &[Faction], next: &mut [Faction]) -> u128 {
    const FACTOR: usize = 4;
    let height = prev.len() / width;
    let corrected = (height / FACTOR) * FACTOR;

    next.par_chunks_mut(width)
        .enumerate()
        .for_each(|(y, chunk)| {
            let mut deck0 = SmallVec::<[_; 9]>::new();
            let mut deck1 = SmallVec::<[_; 9]>::new();
            let mut deck2 = SmallVec::<[_; 9]>::new();
            let mut deck3 = SmallVec::<[_; 9]>::new();
            let mut rng0 = LehmerRng::new(mix(seed, y, LEHMER_MULT0), LEHMER_MULT0);
            let mut rng1 = LehmerRng::new(mix(seed, y, LEHMER_MULT1), LEHMER_MULT1);
            let mut rng2 = LehmerRng::new(mix(seed, y, LEHMER_MULT2), LEHMER_MULT2);
            let mut rng3 = LehmerRng::new(mix(seed, y, LEHMER_MULT3), LEHMER_MULT3);
            let mut calc = |x, deck: &mut SmallVec<[_; 9]>, n| {
                if !deck.is_empty() {
                    let f = select(&deck[..], n);
                    chunk[x] = Faction::Faction(f);
                    deck.clear();
                }
            };
            for x in (0..corrected).step_by(FACTOR) {
                // x x x x x x
                // x o o o o x
                // x x x x x x
                for dy in -1..=1 {
                    for dx in -1..=(FACTOR as i64) {
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
                calc(x + 0, &mut deck0, rng0.gen());
                calc(x + 1, &mut deck1, rng1.gen());
                calc(x + 2, &mut deck2, rng2.gen());
                calc(x + 3, &mut deck3, rng3.gen());
            }
            for x in corrected..height {
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        let xx = x as i64 + dx;
                        let yy = y as i64 + dy;
                        let idx = index(width, xx, yy);
                        if let Some(Faction::Faction(f)) = prev.get(idx) {
                            deck0.push(*f);
                        }
                    }
                }
                calc(x, &mut deck0, rng0.gen());
            }
        });
    seed.wrapping_mul(LEHMER_MULT4)
}
