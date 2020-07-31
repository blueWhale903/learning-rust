// Enums are types which have a few definite values

use std::io;

#[derive(Debug)]
enum Movement {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

fn move_avatar(m: Movement) {
    // Perform action depending on info
    match m {
        Movement::UP => println!("---Avatar moving up---"),
        Movement::DOWN => println!("---Avatar moving down---"),
        Movement::LEFT => println!("---Avatar moving left---"),
        Movement::RIGHT => println!("---Avatar moving right---")
    }
}

fn get_input() -> String {
    use std::io::{stdin,stdout,Write};
    let mut s=String::new();
    print!("Move direction: ");
    stdout().flush();
    stdin().read_line(&mut s);
    if let Some('\n')=s.chars().next_back() {
        s.pop();
    }
    if let Some('\r')=s.chars().next_back() {
        s.pop();
    }

    s
}

fn get_direction(dir: &str) -> Option<Movement> {
    match dir {
        "up" => Some(Movement::UP),
        "down" => Some(Movement::DOWN),
        "left" => Some(Movement::LEFT),
        "right" => Some(Movement::RIGHT),
        _ => None
    }
}

pub fn run() {
    let mut input = String::from("");
    let dir : Movement = Movement::UP;
    let up = String::from("up");
    while true {
        input = get_input().to_lowercase();
        match get_direction(&input) {
            Some(o) => move_avatar(o),
            None => println!("STAND STILL!")
        }
    }
}
