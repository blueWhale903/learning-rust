pub fn run() {
    let age = 22;
    let check_id : bool = true;
    
    println!("Age: {}", age);

    if age >= 21 {
        println!("Bartender: What would you like to drink?");
    } else if age < 21 && check_id {
        println!("Bartender: Sorry, you have to leave");
    } else {
        println!("Bartender: I'll need to see your ID");
    }

    // Shorthand If
    let is_of_age = if age >= 21 {true} else {false};
    println!("is of age: {}", is_of_age);
}