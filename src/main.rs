mod print;
mod string;
mod types;
mod tuples;
mod arrays;

fn main() {
    println!("----------Calling imported 'run' function----------",);
    print::run();

    println!("----------Type----------");
    types::run();

    println!("----------String----------");
    string::run();

    println!("----------Tuples----------");
    tuples::run();

    println!("----------Arrays----------");
    arrays::run();
}
