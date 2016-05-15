use rand::os::OsRng;
use rand::{XorShiftRng, Rand, Rng};
use rand::distributions::{IndependentSample, Range};

use na::Pnt2 as Point2;

use level::Level;

pub struct RoomGen {
    rand_x: XorShiftRng,
    rand_y: XorShiftRng,
    max_room_size: usize,
    min_room_size: usize,
    room_distance: usize,
    attempts: u64,
    rooms: Vec<Room>,
}

impl RoomGen {
    pub fn new(min_room_size: usize, max_room_size: usize, room_distance: u32, attempts :u64) -> RoomGen {
        RoomGen{
            rand_x: XorShiftRng::rand(&mut OsRng::new().unwrap()),
            rand_y: XorShiftRng::rand(&mut OsRng::new().unwrap()),
            max_room_size: max_room_size,
            min_room_size: min_room_size,
            room_distance: room_distance,
            attempts: attempts,
            rooms: Vec::new(),
        }
    }

    pub fn generate(&mut self, level: &mut Level) {
        for i in 0..self.attempts {
            let room = self.generate_box(level);

            let fits = !self.check_collisions(room);
            if fits {
                self.rooms.append(room);
            }
        }
    }

    pub fn reset_rooms(&mut self) {
        self.rooms = Vec::new();
    }

    pub fn generate_box(&mut self, level: &mut Level) -> Box {
        let min_range_x = Range::new(0, level.get_width());
        let min_range_y = Range::new(0, level.get_height());
        let mut min = Point2::new(min_range_x.ind_sample(self.rand_x), min_range_y.ind_sample(self.rand_y));

        let max_range = Range::new(self.min_room_size, self.max_room_size);
        let mut max = Point2::new(max_range.ind_sample(self.rand_x), max_range.ind_sample(self.rand_y));
        max.x += min.x;
        max.y += min.y;
        max.x += self.room_distance;
        max.y += self.room_distance;
        Room{min: min, max: max}
    }

    pub fn check_collisions(&self, room: Room) {

    }
}



struct Room {
    pub min: Point2<isize>,
    pub max: Point2<isize>,
}
