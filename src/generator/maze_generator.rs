use level::Level;

use na::Vec2;

use rand::os::OsRng;
use rand::XorShiftRng;
use rand::Rand;
use rand::Rng;

pub struct MazeGen {
    pos: Vec2<usize>,
    rand: XorShiftRng,
}

impl MazeGen {
    pub fn new(x: usize, y: usize) -> MazeGen {
        MazeGen{
            pos: Vec2::new(x,y),
            rand: XorShiftRng::rand(&mut OsRng::new().unwrap()),
        }
    }

    pub fn get_start_pos(&self) -> &Vec2 {
        self.pos
    }

    pub fn set_start_pos(&mut self, pos: &Vec2) {
        self.pos = pos.clone();
    }

    pub fn generate(&self, level: &mut Level) {
        let mut stack: Vec<&Vec2<usize>> = Vec::new();
        stack.push(self.pos);
        'mainloop loop {
            match stack.pop(){
                Some(pos) => {
                    match level.
                },
                None => continue 'mainloop,
            }
        }
    }


}
