use rand::os::OsRng;
use rand::XorShiftRng;
use rand::Rand;
use rand::Rng;

pub struct RoomGen {
    rand: XorShiftRng,
}
