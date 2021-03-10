extern crate dungenon;

use dungenon::{generator::FactionGen, level::GridLevel, tile::Faction};

#[test]
fn powers_of_two() {
    for i in 1..11 {
        let mut gen = FactionGen::new();
        let mut a = GridLevel::new(1 << i, 1 << i);
        *(a.get_mut_tile(0, 0)
            .unwrap_or_else(|_| panic!("Should exist"))) = Faction::Faction(1);
        *(a.get_mut_tile(a.get_width() - 1, a.get_height() - 1)
            .unwrap_or_else(|_| panic!("Should exist"))) = Faction::Faction(2);
        let mut b = a.clone();
        for _ in 0..50 {
            gen.generate(&mut a, &mut b);
        }
    }
}

#[test]
fn powers_of_two_minus_one() {
    for i in 1..11 {
        let mut gen = FactionGen::new();
        let mut a = GridLevel::new(1 << i - 1, 1 << i - 1);
        *(a.get_mut_tile(0, 0)
            .unwrap_or_else(|_| panic!("Should exist"))) = Faction::Faction(1);
        *(a.get_mut_tile(a.get_width() - 1, a.get_height() - 1)
            .unwrap_or_else(|_| panic!("Should exist"))) = Faction::Faction(2);
        let mut b = a.clone();
        for _ in 0..50 {
            gen.generate(&mut a, &mut b);
        }
    }
}

#[test]
fn non_square() {
    let max = 11;
    for i in 1..max {
        let mut gen = FactionGen::new();
        let mut a = GridLevel::new(1 << i, 1 << (max - i));
        *(a.get_mut_tile(0, 0)
            .unwrap_or_else(|_| panic!("Should exist"))) = Faction::Faction(1);
        *(a.get_mut_tile(a.get_width() - 1, a.get_height() - 1)
            .unwrap_or_else(|_| panic!("Should exist"))) = Faction::Faction(2);
        let mut b = a.clone();
        for _ in 0..50 {
            gen.generate(&mut a, &mut b);
        }
    }
}
