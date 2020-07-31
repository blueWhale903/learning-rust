// Array - fixed list where elements are the same data types

pub fn run() {
    // Initialize an array
    let mut numbers : [i32; 5] = [9; 5];


    println!("numbers array: {:?}", numbers);

    // Get single value
    let first_ele = numbers[0];
    println!("{}", first_ele);

    // Get array length
    let len = numbers.len();
    println!("numbers' length: {}", len);

    //  Arrays are stack allocated
    println!("Array numbers occupies {} bytes", std::mem::size_of_val(&numbers));

    // Get slice
    let slice : &[i32] = &numbers;

    println!("Slice array: {:?}", slice);
    numbers[0] = 10;
    println!("Slice array after change numbers[0]: {:?}", numbers);

    // Loop through array
    for x in numbers.iter() { 
        println!("{}", x);
    }

    // Loop & mutate values
    for x in numbers.iter_mut() {
        *x *= 2;
    }
  
}