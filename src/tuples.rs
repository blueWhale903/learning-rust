// Tuples group together values of different type
// Max 12 elements

pub fn run() {
    // Initialize a tuple
    let mut person : (&str, &str, i16) = ("Brad", "Mass", 199);

    println!("{} is from {} and is {}", person.0, person.1, person.2);
    
    // Modify the tuple
    person.0 = "John";

    println!("{} is from {} and is {}", person.0, person.1, person.2);

}