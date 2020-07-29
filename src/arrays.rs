// Array - fixed list where elements are the same data types

pub fn run() {
    // Initialize an array
    let mut numbers : [i32; 5] = [9; 5];


    println!("Arrays: {:?}", numbers);

    // Get single value
    let first_ele = numbers[0];
    println!("{}", first_ele);

    // Get array length
    let len = numbers.len();
    println!("Array's length: {}", len);

    //  Arrays are stack allocated
    println!("Array occupies {} bytes", std::mem::size_of_val(&numbers));

    // Get slice
    let slice : &[i32] = &numbers;

    println!("Slice array: {:?}", slice);
    numbers[0] = 10;
    println!("Slice array: {:?}", numbers);

    // Loop through array
    for x in numbers.iter() { 
        println!("{}", x);
    }
  
}