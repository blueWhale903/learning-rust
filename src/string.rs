pub fn run() {
    let mut hello = String::from("Hello ");
 
    // Print string length
    println!("Length: {}", hello.len());

    // Push character
    hello.push('W');
    println!("After push a character: {}", hello);

    // Push string
    hello.push_str("orld");
    println!("After push a string: {}", hello);

    // Capacity in bytes
    println!("Capacity: {}", hello.capacity());

    // Check is empty
    println!("Is Empty: {}", hello.is_empty());

    // Contains
    println!("Contains 'World': {}", hello.contains("World"));

    // Replace
    println!("Replace 'World' with 'there': {}",
            hello.replace("World","there!"));

    // Loop through string by whitespace 
    for word in hello.split_whitespace() {
        println!("{}", word )
    }

    // Create string with capacity
    let mut s = String::with_capacity(1);
    s.push('a');
    s.push('b');
    println!("{}", s.capacity());

    // Assertion testing
    assert_eq!(2, s.len());
    // assert_eq!(3, s.len());
}
