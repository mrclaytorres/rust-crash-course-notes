// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - Use when you need to modify or own string data

pub fn run() {
    let mut hello = String::from("Hello ");

    // Get length
    println!("Length: {}", hello.len());

    // Push Char
    hello.push('W');

    hello.push_str("orld!");

    // Capacity in bytes
    println!("Capacity: {}", hello.capacity());

    // Check if string is empty
    println!("Is empty: {}", hello.is_empty());

    // Contains some substring
    println!("Contains 'World' {}", hello.contains("world"));

    //Replace
    println!("Replace {}", hello.replace("World", "There"));

    // Loop through string by whitespace
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    // Create string with capacity
    let mut s = String::with_capacity(10);

    s.push('a');
    s.push('b');

    // Assertion
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    println!("{}", s);
}