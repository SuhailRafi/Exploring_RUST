// No Fixed length
// Same data type

use std::mem;

pub fn run () {
    let mut numbers: Vec<i32> = vec![1,2,3,4]; 
    
    // Re-assign values
    numbers [2] = 20;

    println!("{:?}", numbers);

    // Add onto vectors
    numbers.push(5);
    numbers.push(6);

    // Pop off the last value
    numbers.pop();

    //Loop through vector values
    for x in numbers.iter (){
        println! ("number: {}", x);
    }

    // Loop and mutate values
    for x in numbers.iter_mut () {
        *x *=2;
    }

    println! ("Numbers Vec {:?}", numbers);


    // Get single val
    println!("Single value: {}", numbers[0]);

    //Get vector length
    println!("Vector Length: {}", numbers.len());
    println!("{:?}", numbers);

    // vector are stack allocated
    println!("Vectors occupies {} bytes", mem::size_of_val(&numbers));

    // Get Slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice); 
}