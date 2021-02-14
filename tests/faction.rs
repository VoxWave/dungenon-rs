extern crate dungenon;

use dungenon::{generator::FactionGen, level::GridLevel};

#[test]
fn faction2() {
    let mut gen = FactionGen::new();
    let mut a = GridLevel::new(1024, 1024);
    let mut b = a.clone();
    for _ in 0..100 {
        gen.generate(&mut a, &mut b);
    }
}

#[test]
fn faction2_odd() {
    let mut gen = FactionGen::new();
    let mut a = GridLevel::new(5, 5);
    let mut b = a.clone();
    for _ in 0..100 {
        gen.generate(&mut a, &mut b);
    }
}
