#[derive(Debug)]
enum Movement {
    up,
    down,
    left,
    right
}

fn move_avatar(mv: Option<Movement>) {
    if let Some(m) = mv {
        println!("Avatar moving {:?}!", m);
    } else {
        println!("Avatar stand still!")
    }
}

fn get_direction(dir: &str) -> Option<Movement> {
    match dir {
        "up" => Some(Movement::up),
        "down" => Some(Movement::down),
        "left" => Some(Movement::left),
        "right" => Some(Movement::right),
        _ => None
    }
}

fn get_input() -> String {
    use std::io::{stdin,stdout,Write};
    let mut s=String::new();
    print!("Move direction: ");
    stdout().flush().expect("Output Error!");
    stdin().read_line(&mut s).expect("Input Error!");
    if let Some('\n')=s.chars().next_back() {
        s.pop();
    }
    if let Some('\r')=s.chars().next_back() {
        s.pop();
    }

    return s
}

pub fn run() {
    let mut is_exit = false;
    let mut input = String::from("");
    let mut dir: Option<Movement>;

    while !is_exit {
        input = get_input();
        dir = get_direction(&input);

        if input == "stop" {
            is_exit = true;
        } else {
            move_avatar(dir);
        }
    }
}
