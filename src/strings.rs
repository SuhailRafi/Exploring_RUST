// Primitive str = Immutable fixed length
// String = Growable, heal allocated data structure

pub fn run () {
    //let hello= "Hello"; // Immutable fixed lenght 
    let mut hello =String::from("Hello "); 

    // Get length
    println! ("length: {}", hello.len());

    //push a char in string
    hello.push('W'); 

    //push a String
    hello.push_str ("orld"); 

    //Capacity in bytes
    println! ("Capacity: {}", hello.capacity());
    // Check if empty
    println! ("Is Empty:: {}", hello.is_empty());

    // Contains
    println! ("Contains world: {}", hello.contains("World!"));

    //Replace
    println! ("Replace: {}", hello.replace("World", "There"));

    // Loop through string by white space

    for word in hello.split_whitespace (){
        println!("{}", word);
    }

    // Create String with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    println!("{}", s);
    println! ("{}", hello);

    // Assertion testing
    assert_eq!(2, s.len());
    assert_eq! (10, s.capacity());

}