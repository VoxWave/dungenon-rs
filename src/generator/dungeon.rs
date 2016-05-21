use generator::{MazeGen, RoomGen};
use level::Level;
use level::fill_dead_ends;

pub struct DungeonGen {
    mazegen: MazeGen,
    roomgen: RoomGen,
}

impl DungeonGen {
    pub fn new(mazegen: MazeGen, roomgen: RoomGen) -> DungeonGen {
        DungeonGen{
            mazegen: mazegen,
            roomgen: roomgen,
        }
    }

    pub fn generate(&mut self, level: &mut Level) {
        level.apply(|m| self.mazegen.generate(m))
        .apply(|m| self.roomgen.generate(m));

        while fill_dead_ends(level) {}
    }
}
