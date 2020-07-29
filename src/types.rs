pub fn run() {
    // Default int is "i32"
    let int_num = 1;

    // Default float is "f64"
    let float_num = 7.9;

    // Add explicit type
    let big_int : i64 = 9999999999999;

    // Find max size
    println!("Max of i32: {}", std::i32::MAX);
    println!("Max of i64: {}", std::i64::MAX);

    // Boolean
    let is_active = false;

    // UTF8
    let face = '\u{1F600}';

    println!("{:?}", (int_num, float_num, big_int, is_active, face))


}