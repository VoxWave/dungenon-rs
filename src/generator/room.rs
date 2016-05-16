use rand::os::OsRng;
use rand::{XorShiftRng, Rand};
use rand::distributions::{IndependentSample, Range};

use na::Pnt2 as Point2;

use tile::Tile;

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
    pub fn new(min_room_size: usize, max_room_size: usize, room_distance: usize, attempts :u64) -> RoomGen {
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
        for _ in 0..self.attempts {
            let room = self.generate_box(level);

            let fits = !self.check_collisions(&room);
            if fits {
                self.rooms.push(room);
            }
        }
        self.carve(level);
    }

    pub fn reset_rooms(&mut self) {
        self.rooms = Vec::new();
    }

    fn generate_box(&mut self, level: &mut Level) -> Room {
        let min_range_x = Range::new(0, level.get_width());
        let min_range_y = Range::new(0, level.get_height());
        let min = Point2::new(min_range_x.ind_sample(&mut self.rand_x), min_range_y.ind_sample(&mut self.rand_y));

        let max_range = Range::new(self.min_room_size, self.max_room_size);
        let mut max = Point2::new(max_range.ind_sample(&mut self.rand_x), max_range.ind_sample(&mut self.rand_y));
        max.x += min.x;
        max.y += min.y;
        max.x += self.room_distance;
        max.y += self.room_distance;

        Room{min: min, max: max}
    }

    fn check_collisions(&self, room: &Room) -> bool {
        for b in &self.rooms {
            if room.overlaps(b) {
                return true;
            }
        }
        false
    }

    fn carve(&self, level: &mut Level) {
        let room_distance = self.room_distance.clone();
        for room in &self.rooms {
            for y in room.min.y..room.max.y-room_distance+1 {
                for x in room.min.x..room.max.x-room_distance+1 {
                    match level.get_mut_tile(x, y) {
                        Some(tile) => *tile = Tile::Floor,
                        None => {}
                    }
                }
            }
        }
    }
}



struct Room {
    pub min: Point2<usize>,
    pub max: Point2<usize>,
}

impl Room {
    pub fn overlaps(&self, room: &Room) -> bool {
        if self.min.x > room.max.x || room.min.x > self.max.x {
            return false;
        }

        if self.min.y > room.max.y || room.min.y > self.max.y {
            return false;
        }
        true
    }
}
