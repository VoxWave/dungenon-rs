extern crate image;
extern crate dungenon;
extern crate cast;
extern crate rand;

use std::io;
use std::path::PathBuf;

use dungenon::level::GridLevel as Level;
use dungenon::tile::{Faction};
use dungenon::generator::{FactionGen};
use dungenon::util::Error;

use rand::os::OsRng;
use rand::XorShiftRng;
use rand::Rand;
use rand::distributions::{IndependentSample, Range};

use image::RgbImage;
use image::Rgb;

use std::collections::HashMap;

fn main() {
    let mut faction_to_color = HashMap::new();
    let mut color_to_faction = HashMap::new();
    let mut index = 0;
    for r in 0..256 {
        for g in 0..256 {
            for b in 0..256 {
                faction_to_color.insert(Faction::Faction(index), Rgb([r as u8, g as u8, b as u8]));
                color_to_faction.insert(Rgb([r as u8, g as u8, b as u8]), Faction::Faction(index));
                index += 1;
            }
        }
    }
    println!("Initializing level...");
    println!("Input level width: ");
    let x = usize_from_cmd();


    println!("Input level height: ");
    let y = usize_from_cmd();

    let mut level = Level::new_filled_with(Faction::Neutral, x, y);
    populate_level(&mut level, index);
    let mut factiongen = FactionGen::new();
    println!("How many iterations");
    let iterations = usize_from_cmd();
    let mut buffer = level.clone();
    for i in 0..iterations {
        factiongen.generate(&mut level, &mut buffer);
        println!("{} iterations done.", i);
    }
    faction_png_export(String::from("picture"), &faction_to_color, &mut level);
}

fn load_level_from_file(path: PathBuf) {

}

fn populate_level(level: &mut Level<Faction>, rangemax: usize) {
    let range = Range::new(0,rangemax);
    let mut rand = XorShiftRng::rand(&mut OsRng::new().unwrap());
    for x in 0..level.get_width() {
        for y in 0..level.get_height() {
            match level.get_mut_tile(x, y) {
                Ok(t) => {
                    *t = Faction::Faction(range.ind_sample(&mut rand));
                },
                _ => unreachable!("aaaa"),
            }
        }
    }
}

fn faction_png_export(name: String, colors: &HashMap<Faction, Rgb<u8>>, level: &mut Level<Faction>) {
    let mut level_image = RgbImage::new(level.get_width() as u32, level.get_height() as u32);
    for x in 0 .. level.get_width() {
        for y in 0 .. level.get_height() {
            level_image.put_pixel(x as u32, y as u32, faction_to_color(colors, &level.get_tile(x,y)));
        }
    }
    let mut p = PathBuf::new();
    p.push(name);
    p.set_extension("png");
    level_image.save(p.as_path()).expect("Something went wrong when saving the png.");
}

fn faction_to_color(colors: &HashMap<Faction, Rgb<u8>>, tile: &Result<&Faction, Error>) -> Rgb<u8> {
    match *tile {
        Ok(ref tile) => {
            match colors.get(tile) {
                Some(color) => color.clone(),
                None => unreachable!("aaaaa!!!"),
            }
        },
        Err(Error::IndexOutOfBounds) => panic!("Tile png import failed. Level indexing went out of bounds."),
    }
}

fn usize_from_cmd() -> usize {
    let mut num = String::new();

    io::stdin().read_line(&mut num)
        .expect("failed to read line");

    let num: usize = num.trim().parse()
        .expect("Please type a number!");
    num
}
