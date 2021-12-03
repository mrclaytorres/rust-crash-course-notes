// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run() {
    let name = "Algen";
    let mut age = 35;
    println!("My name is {} and I am {}", name, age);
    age = 34;
    println!("My name is {} and I am {}", name, age);

    // Define constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign multiple variables
    let (my_name, mut my_age) = ("Algen", 35);

    my_age = 36;
    println!("{} is {}", my_name, my_age);
}
