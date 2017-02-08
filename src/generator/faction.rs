use rand::os::OsRng;
use rand::XorShiftRng;

pub struct FactionGen {
    rand: XorShiftRng,
    faction_amount: usize.
}

impl FactionGen {
	pub fn new(faction_amount: usize) {
		rand: XorShiftRng::rand(&mut OsRng::new().unwrap()),
		faction_amount: faction_amount,
	}

	pub fn generate(&mut self, level: &mut Level<Faction>) {
		for x in 0..level.get_width() {
			for y in 0..level.get_height() {
				match level.get_mut_tile(x, y) {
					Ok(tile) => {
						let mut deck = Vec::new();
						match *tile {
							Faction::Faction(_) => {
								deck.push(tile);
							},
							Faction::Void => continue,
							_ => {},
						}
						get_faction_neighbours(&mut deck, &mut level);
						self.rand.shuffle(deck);
						*tile = deck.pop();
					},
					Err(Error::IndexOutOfBounds) => {
						panic!("Generate method indexed out of bounds while simulating a step. This should never happen unless the programmer is not very bright.");
					}
				}
			}
		}
	}
}