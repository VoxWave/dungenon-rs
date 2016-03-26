extern crate image;
extern crate dungenon;
extern crate cast;

use std::io;
use std::path::PathBuf;

use dungenon::level::GridLevel as Level;
use dungenon::tile::{Tile, Faction};
use dungenon::generator::{MazeGen, RoomGen, DungeonGen, FactionGen};
use dungenon::util::Error;

use image::RgbImage;
use image::Rgb;

use std::collections::HashMap;

fn main() {
    Drawer::new().start();
}

struct Drawer {
    pub dungeon_colors: HashMap<Tile, Rgb<u8>>,
    pub faction_colors: HashMap<Faction, Rgb<u8>>,
}

impl Drawer {
    pub fn new() -> Drawer {
        let mut dungeon_colors = HashMap::new();
        dungeon_colors.insert(Tile::Wall(0), Rgb([127u8, 127u8, 127u8]));
        dungeon_colors.insert(Tile::Floor(0), Rgb([200u8, 200u8, 200u8]));
        dungeon_colors.insert(Tile::Void(0), Rgb([0u8, 0u8, 0u8]));

        let mut faction_colors = HashMap::new();
        faction_colors.insert(Faction::Faction(0), Rgb([0u8, 0u8, 0u8]));
        faction_colors.insert(Faction::Faction(1), Rgb([255u8, 0u8, 0u8]));
        faction_colors.insert(Faction::Faction(2), Rgb([0u8, 255u8, 0u8]));
        faction_colors.insert(Faction::Faction(3), Rgb([255u8, 255u8, 0u8]));
        faction_colors.insert(Faction::Faction(4), Rgb([0u8, 0u8, 255u8]));
        faction_colors.insert(Faction::Faction(5), Rgb([255u8, 0u8, 255u8]));
        faction_colors.insert(Faction::Faction(6), Rgb([0u8, 255u8, 255u8]));
        faction_colors.insert(Faction::Faction(7), Rgb([255u8, 255u8, 255u8]));
        faction_colors.insert(Faction::Neutral, Rgb([127u8, 127u8, 127u8]));
        faction_colors.insert(Faction::Void, Rgb([0u8, 0u8, 0u8]));

        Drawer{
            dungeon_colors: dungeon_colors,
            faction_colors: faction_colors,
        }
    }

    pub fn start(&mut self) {
        loop {
            println!("Choose level type(faction, tile):");
            let t = Self::string_from_cmd();
            match t.trim() {
                "faction" => self.start_faction(),
                "tile" => self.start_tile(),
                _ => continue,
            }
            break;
        }
    }

    pub fn start_faction(&mut self) {
        let mut level = self.init_faction_level();
        let mut simulator = FactionGen::new();

        println!("This is the faction mode of dungenon-drawer.");
        println!("Type Help for list of possible commands.");
        loop {
            println!("Enter command:");
            let command = Self::string_from_cmd();
            match command.trim() {
                "simulate" => Self::simulate_factions(&mut level, &mut simulator),
                "factions" => Self::edit_level(&mut level),
                "colors" => self.change_faction_colors(),
                "export" => self.faction_png_export(&mut level),
                "help" => println!("Options: help, simulate, factions, colors, export and exit"),
                "exit" => break,
                _ => println!("Invalid command.")
            }
        }
    }

    fn change_faction_colors(&mut self) {
        use cast;
        use std::u8;
        loop {
            println!("Which color would you like to change? (faction, neutral, void), Type exit if you're done with modifying colors.");
            let command = Self::string_from_cmd();
            let key = match command.trim() {
                "faction" => {
                    println!("Enter faction number:");
                    Faction::Faction(Self::usize_from_cmd())
                },
                "neutral" => Faction::Neutral,
                "void" => Faction::Void,
                "exit" => break,
                _ => {
                    println!("Invalid command.");
                    continue;
                },
            };
            println!("Input red value for the color:");
            let r = cast::u8(Self::usize_from_cmd()).unwrap_or(u8::MAX);

            println!("Input green value for the color:");
            let g = cast::u8(Self::usize_from_cmd()).unwrap_or(u8::MAX);

            println!("Input blue value for the color:");
            let b = cast::u8(Self::usize_from_cmd()).unwrap_or(u8::MAX);
            self.faction_colors.insert(key, Rgb([r,g,b]));
        }
    }

    fn init_faction_level(&self) -> Level<Faction> {
        println!("Initializing level...");
        println!("Input level width: ");
        let x = Self::usize_from_cmd();


        println!("Input level height: ");
        let y = Self::usize_from_cmd();

        Level::new_filled_with(Faction::Neutral, x, y)
    }

    fn simulate_factions(level: &mut Level<Faction>, simulator: &mut FactionGen) {
        println!("How many simulation steps are iterated?");
        let mut buffer = level.clone();
        let iterations = Self::u64_from_cmd();
        let percent = iterations/100;
        for n in 0 .. iterations {
            simulator.generate(level, &mut buffer);
            if n % percent == 0 {
                println!("{}%", n/percent);
            }
        }
    }

    fn faction_png_export(&self, level: &mut Level<Faction>) {
        let mut level_image = RgbImage::new(level.get_width() as u32, level.get_height() as u32);
        for x in 0 .. level.get_width() {
            for y in 0 .. level.get_height() {
                level_image.put_pixel(x as u32, y as u32, self.faction_to_color(&level.get_tile_with_tuple((x,y))));
            }
        }
        let mut p = PathBuf::new();
        println!("Enter png name:");
        p.push(Self::string_from_cmd().trim());
        p.set_extension("png");
        level_image.save(p.as_path()).expect("Something went wrong when saving the png.");
    }

    fn edit_level(level: &mut Level<Faction>) {
        loop {
            println!("Input x coordinate:");
            let x = Self::usize_from_cmd();
            println!("Input y coordinate:");
            let y = Self::usize_from_cmd();

            println!("Select type(faction, neutral, void):");
            let faction_type = Self::string_from_cmd();
            let faction = match faction_type.trim() {
                "faction" => {
                    println!("Input faction number:");
                    let faction_number = Self::usize_from_cmd();
                    Faction::Faction(faction_number)
                },
                "neutral" => Faction::Neutral,
                "void" => Faction::Void,
                _ => {
                    println!("Invalid type.");
                    break;
                }
            };
            match level.get_mut_tile(x,y) {
                Ok(tile) => {
                    *tile = faction;
                },
                Err(_) => println!("Index out of bounds"),
            };
            println!("do you want to change another tile? (yes/no)");
            match Self::string_from_cmd().trim() {
                "yes" => continue,
                "no" => break,
                _ => {println!("Invalid input. Interpretting")}
            }
        }
    }

    pub fn start_tile(&mut self) {
        let mut level = self.init_tile_level();

        println!("This is the tile mode of dungenon-drawer. \nType Help for list of commands.");
        loop {
            println!("Enter command:");
            let command = Self::string_from_cmd();
            match command.trim() {
                "dungeon" => Self::carve_dungeon(&mut level),
                "maze" => Self::carve_maze(&mut level),
                "room" => Self::carve_rooms(&mut level),
                "reset" => level = self.init_tile_level(),
                "colors" => self.change_dungeon_colors(),
                "export" => {
                    self.tile_png_export(&mut level);
                },
                "print" => Self::print_level(&level),
                "help" => {
                    println!("Options: dungeon, maze, room, reset, colors, export, print, help, and exit");
                }
                "exit" => break,
                _ => println!("Invalid command."),
            }
        }
    }

    fn faction_to_color(&self, tile: &Result<&Faction, Error>) -> Rgb<u8> {
        match *tile {
            Ok(ref tile) => {
                    match self.faction_colors.get(tile) {
                        Some(color) => color.clone(),
                        None => Rgb([255u8, 0u8, 255u8]),
                    }
            },
            Err(Error::IndexOutOfBounds) => panic!("Tile png import failed. Level indexing went out of bounds."),
        }
    }

    fn change_dungeon_colors(&mut self) {
        use cast;
        use std::u8;
        loop {
            println!("Which color would you like to change? (floor, wall, void), Type exit if you're done with modifying colors.");
            let command = Self::string_from_cmd();
            let key = match command.trim() {
                "floor" => Tile::Floor(0),
                "wall" => Tile::Wall(0),
                "void" => Tile::Void(0),
                "exit" => break,
                _ => {
                    println!("Invalid command.");
                    continue;
                },
            };
            println!("Input red value for the color:");
            let r = cast::u8(Self::usize_from_cmd()).unwrap_or(u8::MAX);

            println!("Input green value for the color:");
            let g = cast::u8(Self::usize_from_cmd()).unwrap_or(u8::MAX);

            println!("Input blue value for the color:");
            let b = cast::u8(Self::usize_from_cmd()).unwrap_or(u8::MAX);
            self.dungeon_colors.insert(key, Rgb([r,g,b]));
        }
    }

    fn init_tile_level(&self) -> Level<Tile> {
        println!("Initializing level...");
        println!("Input level width: ");
        let x = Self::usize_from_cmd();


        println!("Input level height: ");
        let y = Self::usize_from_cmd();

        Level::new_filled_with(Tile::Wall(0), x, y)
    }

    fn carve_dungeon(level: &mut Level<Tile>) {
        println!("Creating DungeonGen...");
        let mazegen = Self::create_mazegen();
        let roomgen = Self::create_roomgen();
        let mut dungeongen = DungeonGen::new(mazegen, roomgen);
        level.apply(|m| dungeongen.generate(m));
    }

    fn create_mazegen() -> MazeGen {
        println!("Input MazeGen startpos x coordinate: ");
        let x = Self::usize_from_cmd();

        println!("Input MazeGen startpos y coordinate: ");
        let y = Self::usize_from_cmd();
        MazeGen::new(x,y)
    }

    fn carve_maze(level: &mut Level<Tile>) {
        let mut mazegen = Self::create_mazegen();
        level.apply(|m| mazegen.generate(m));
    }

    fn create_roomgen() -> RoomGen {
        println!("Input min room size:");
        let min_room_size = Self::usize_from_cmd();

        println!("Input max room size:");
        let max_room_size = Self::usize_from_cmd();

        println!("Input min room distance:");
        let room_distance = Self::usize_from_cmd();

        println!("Input room placement amount (Something high preferably):");
        let attempts = Self::u64_from_cmd();

        RoomGen::new(min_room_size, max_room_size, room_distance, attempts)
    }

    fn carve_rooms(level: &mut Level<Tile>) {
        let mut roomgen = Self::create_roomgen();
        level.apply(|m| roomgen.generate(m));
    }

    fn tile_png_export(&self, level: &mut Level<Tile>) {
        let mut level_image = RgbImage::new(level.get_width() as u32, level.get_height() as u32);
        for x in 0 .. level.get_width() {
            for y in 0 .. level.get_height() {
                level_image.put_pixel(x as u32, y as u32, self.tile_to_color(&level.get_tile_with_tuple((x,y))));
            }
        }
        let mut p = PathBuf::new();
        println!("Enter png name:");
        p.push(Self::string_from_cmd().trim());
        p.set_extension("png");
        level_image.save(p.as_path()).expect("Something went wrong when saving the png.");
    }

    fn tile_to_color(&self, tile: &Result<&Tile, Error>) -> Rgb<u8> {
        match *tile {
            Ok(ref tile) => {
                    match self.dungeon_colors.get(tile) {
                        Some(color) => color.clone(),
                        None => Rgb([255u8, 0u8, 255u8]),
                    }
            },
            Err(Error::IndexOutOfBounds) => panic!("Tile png import failed. Level indexing went out of bounds."),
        }
    }


    fn print_level(level: &Level<Tile>) {
        use dungenon::util::Error;
        let mut string = String::new();
        for y in 0 .. level.get_width() {
            for x in 0 .. level.get_height() {
                match level.get_tile_with_tuple((x,y)) {
                    Ok(tile) => {
                        match tile {
                            &Tile::Wall(_) => string.push('#'),
                            &Tile::Floor(_) => string.push(' '),
                            &Tile::Void(_) => string.push('*'),
                        }
                    },
                    Err(Error::IndexOutOfBounds) => panic!("IndexOutOfBounds occurred. Level printing wasn't implemented properly."),
                }
            }
            string.push('\n');
        }
        println!("{}", &string);
    }

    fn usize_from_cmd() -> usize {
        let mut num = String::new();

        io::stdin().read_line(&mut num)
            .expect("failed to read line");

        let num: usize = num.trim().parse()
            .expect("Please type a number!");
        num
    }

    fn u64_from_cmd() -> u64 {
        let mut num = String::new();

        io::stdin().read_line(&mut num)
            .expect("failed to read line");

        let num: u64 = num.trim().parse()
            .expect("Please type a number!");
        num
    }

    fn string_from_cmd() -> String {
        let mut string = String::new();
        io::stdin().read_line(&mut string)
        .expect("Failed to read line");
        string
    }
}
