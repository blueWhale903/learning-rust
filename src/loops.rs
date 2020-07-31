pub fn run() {
    let mut count = 0;

    // Infinite loop
    // loop {
    //     count += 1;
    //     println!("Number: {}", count);

    //     if count == 20 {
    //         break;
    //     }
    // }

    // While loop
    println!("While loop");
    while count <= 100 {
        if count % 15 == 0 {
            println!("{} is multiple of 15", count);
        }
        count += 1;
    }
    // For range
    let range = 100;
    println!("For range");
    for x in 0..range {
        if x % 15 == 0 {
            println!("{} is multiple of 15", x);
        }
        // count += 1;
    }
}