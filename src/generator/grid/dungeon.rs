use generator::{MazeGen, RoomGen};
use level::GridLevel;
use level::fill_dead_end_tiles;
use tile::Tile;

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

    pub fn generate(&mut self, level: &mut GridLevel<Tile>) {
        level.apply(|m| self.mazegen.generate(m))
        .apply(|m| self.roomgen.generate(m));

        while fill_dead_end_tiles(level) {}
    }
}
