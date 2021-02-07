use Point;

use rand::{
    distributions::{Distribution, Uniform},
    rngs::OsRng,
    SeedableRng, XorShiftRng,
};
use tile::Tile;

use level::GridLevel;

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
    pub fn new(
        min_room_size: usize,
        max_room_size: usize,
        room_distance: usize,
        attempts: u64,
    ) -> RoomGen {
        RoomGen {
            rand_x: XorShiftRng::from_rng(&mut OsRng::new().unwrap()).unwrap(),
            rand_y: XorShiftRng::from_rng(&mut OsRng::new().unwrap()).unwrap(),
            max_room_size: max_room_size,
            min_room_size: min_room_size,
            room_distance: room_distance,
            attempts: attempts,
            rooms: Vec::new(),
        }
    }

    pub fn generate(&mut self, level: &mut GridLevel<Tile>) {
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

    fn generate_box(&mut self, level: &mut GridLevel<Tile>) -> Room {
        let min_range_x = Uniform::new(0, level.get_width());
        let min_range_y = Uniform::new(0, level.get_height());
        let min = Point::new(
            min_range_x.sample(&mut self.rand_x),
            min_range_y.sample(&mut self.rand_y),
        );

        let max_range = Uniform::new(self.min_room_size, self.max_room_size);
        let mut max = Point::new(
            max_range.sample(&mut self.rand_x),
            max_range.sample(&mut self.rand_y),
        );
        max.x += min.x;
        max.y += min.y;
        max.x += self.room_distance;
        max.y += self.room_distance;

        Room { min: min, max: max }
    }

    fn check_collisions(&self, room: &Room) -> bool {
        for b in &self.rooms {
            if room.overlaps(b) {
                return true;
            }
        }
        false
    }

    fn carve(&self, level: &mut GridLevel<Tile>) {
        use util::Error;
        let room_distance = self.room_distance.clone();
        for room in &self.rooms {
            for y in room.min.y..room.max.y - room_distance + 1 {
                for x in room.min.x..room.max.x - room_distance + 1 {
                    match level.get_mut_tile(x, y) {
                        Ok(tile) => *tile = Tile::Floor(0),
                        Err(Error::IndexOutOfBounds) => {}
                    }
                }
            }
        }
    }
}

struct Room {
    pub min: Point<usize>,
    pub max: Point<usize>,
}

impl Room {
    pub fn overlaps(&self, room: &Room) -> bool {
        self.min.x <= room.max.x
            && room.min.x <= self.max.x
            && self.min.y <= room.max.y
            && room.min.y <= self.max.y
    }
}
