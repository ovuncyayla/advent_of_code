// Advent of Code - Day 2
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Command(String, i32);

#[derive(Debug)]
struct Position {
    x: i32,
    y: i32, // aim
}

struct Submarine {
    position: Position,
    depth: i32
}

impl Submarine {
    pub fn new() -> Submarine {
        Submarine {
            position: Position { x: 0, y: 0 },
            depth: 0
        }
    }

    pub fn process_cmd(&mut self, cmd: Command) {
        match cmd.0.as_str() {
            "forward" => {
                self.position.x += cmd.1;
                self.depth += self.position.y * cmd.1
            }
            "down" => self.position.y += cmd.1,
            "up" => self.position.y -= cmd.1,
            _ => unreachable!(),
        }
    }

    pub fn multiplied_depth(&self) -> i32 {
        self.position.x * self.depth
    }
}

fn main() {
    // Read input
    let input = std::fs::read_to_string("./input.ron").unwrap();
    let inp: Vec<Command> = ron::from_str(input.as_str()).unwrap();

    let mut submarine = Submarine::new();
    for cmd in inp {
        submarine.process_cmd(cmd);
    }

    println!("Final pos: {:?}", submarine.position);
    println!("Final depth: {:?}", submarine.depth);
    println!("Final pos multiplied: {:?}", submarine.multiplied_depth());
}

