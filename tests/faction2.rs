extern crate dungenon;

use dungenon::{generator::FactionGen2, level::GridLevel};

#[test]
fn faction2() {
    let mut gen = FactionGen2::new();
    let mut a = GridLevel::new(1028, 1028);
    let mut b = a.clone();
    for _ in 0..100 {
        gen.generate(&mut a, &mut b);
    }
}
