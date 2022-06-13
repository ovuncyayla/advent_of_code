// Advent of Code - Day 2
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Command(String, i32);

#[derive(Debug)]
struct Position {
    x: i32,
    y: i32,
}

struct Submarine {
    position: Position,
}

impl Submarine {
    pub fn new() -> Submarine {
        Submarine {
            position: Position { x: 0, y: 0 },
        }
    }

    pub fn process_cmd(&mut self, cmd: Command) {
        match cmd.0.as_str() {
            "forward" => self.position.x += cmd.1,
            "down" => self.position.y += cmd.1,
            "up" => self.position.y -= cmd.1,
            _ => unreachable!(),
        }
    }

    pub fn multiplied_pos(&self) -> i32 {
        self.position.x * self.position.y
    }
}

fn main() {
    // Read input
    let input = std::fs::read_to_string("./input.ron").unwrap();
    let inp: Vec<Command> = ron::from_str(input.as_str()).unwrap();

    let mut submarine = Submarine::new();
    for cmd in inp {
        submarine.process_cmd(cmd);
        //println!("{:#?}", cmd);
    }

    println!("Final pos: {:?}", submarine.position);
    println!("Final pos multiplied: {:?}", submarine.multiplied_pos());
}
