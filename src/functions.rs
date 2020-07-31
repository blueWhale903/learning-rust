pub fn run() {
    greeting("Hello", "Ben");
    println!("Sum of 13 and 27: {}", add(13, 27));

    // Closure
    let n3: i32 = 10;
    let add_num = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("Closure function: {}", add_num(13, 27));
}

pub fn greeting(greet: &str, name: &str) {
    println!("{} {}, nice to meet you!", greet, name);
}

fn add(num1: i32, num2: i32) -> i32 {
    return num1 + num2;
}