use rand::os::OsRng;
use rand::XorShiftRng;
use rand::Rand;
use rand::Rng;

use na::Point2;

use level::Level;

pub struct RoomGen {
    rand_x: XorShiftRng,
    rand_y: XorShiftRng,
    max_room_size: usize,
    min_room_size: usize,
    attempts: u64,
    rooms: Vec<Box>,
}

impl RoomGen {
    pub fn new(min_room_size: usize, max_room_size: usize, room_distance: u32, attempts :u64) -> RoomGen {
        RoomGen{
            rand_x: XorShiftRng::rand(&mut OsRng::new().unwrap()),
            rand_y: XorShiftRng::rand(&mut OsRng::new().unwrap()),
            max_room_size: max_room_size,
            min_room_size: min_room_size,
            attempts: attempts,
            rooms: Vec::new(),
        }
    }

    pub fn generate(&mut self, level: &mut Level) {
        for i in 0..self.attempts {
            let room = self.generate_box(level);

            let fits = !self.check_collisions(room);
            if fits {
                rooms.append(room);
            }
        }
    }

    pub fn reset_rooms(&mut self) {
        self.rooms = Vec::new();
    }

    pub fn generate_box(&mut self, level: &mut Level) -> Box {
        let mut min = Point2::new(self.rand_x.);
    }

    pub fn check_collisions(&self, room: Room) {

    }
}



struct Room {
    pub min: Point2<isize>,
    pub max: Point2<isize>,
}
