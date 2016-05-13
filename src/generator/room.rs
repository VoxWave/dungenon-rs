use rand::os::OsRng;
use rand::XorShiftRng;
use rand::Rand;
use rand::Rng;

use na::Vec2;

use level::Level;

pub struct RoomGen {
    rand_x: XorShiftRng,
    rand_y: XorShiftRng,
    max_room_size: usize,
    min_room_size: usize,
    attempts: u64,
}

impl RoomGen {
    pub fn new(min_room_size: usize, max_room_size: usize, room_distance: u32, attempts :u64) -> RoomGen {
        RoomGen{
            rand_x: XorShiftRng::rand(&mut OsRng::new().unwrap()),
            rand_y: XorShiftRng::rand(&mut OsRng::new().unwrap()),
            max_room_size: max_room_size,
            min_room_size: min_room_size,
            attempts: attempts,
        }
    }

    pub fn generate(&mut self, level: &mut Level) {
        let mut rooms = Vec::new();
        for i in 0..self.attempts {
            let room = Self::generate_box(level);

            let fits = !self.check_collisions();
            if fits {
                rooms.append(room);
            }
        }
    }

    pub fn generate_box(level: &mut Level) -> Box {
    }
}



struct Box {
    pub min: Vec2<isize>,
    pub max:Vec2<isize>,
}
