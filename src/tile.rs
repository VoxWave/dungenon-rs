#[derive(Clone, Debug, Eq, PartialEq, Hash)]
//the indexes inside the enums are for flexibility. You can create an extra array for different
//types of particular tile type and index it with the index. For example you could have an array of
//different wall texture variations and have your level generator algorithm put random indexes in
//the walls so that the level would be more visually varied. You could also also have an array of
//different types of floors which each have their own properties (normal floor, icy floor, lava floor etc.)
pub enum Tile {
    Wall(usize),
    Floor(usize),
    Void(usize),
}

impl Default for Tile {
    fn default() -> Tile {
        Tile::Void(0)
    }
}
