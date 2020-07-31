// Vectors are resizable array

pub fn run() {
    // Initialize an array
    let mut numbers : Vec<i32> = [0; 10].to_vec();


    println!("Vector 'numbers': {:?}", numbers);

    // Push a value
    numbers.push(2);
    println!("Push a value: {:?}", numbers);

    // Get single value
    let first_ele = numbers[0];
    println!("{}", first_ele);

    // Get array length
    let len = numbers.len();
    println!("numbers' length: {}", len);

    //  Vectors are stack allocated
    println!("Vector numbers occupies {} bytes", std::mem::size_of_val(&numbers));

    // Get slice
    let slice : &[i32] = &numbers;

    println!("Slice array: {:?}", slice);
    numbers[0] = 10;
    println!("Slice array after change numbers[0]: {:?}", numbers);

    // Loop through array
    for x in numbers.iter() { 
        println!("{}", x);
    }
  
}