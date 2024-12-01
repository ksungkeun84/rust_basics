// Data Types

fn main() {
    // Rust is a statically typed language
    // guess must be annotated with data type so that
    // parse function can determine the type at compile time.
    let guess: u32 = "42".parse().expect("Not a number!");

    println!("guess: {guess}");

    // Scala Types are
    // Integer, Floating-point, boolean, character types

    // Compound Types
    // Tuple: a group of values with a variety of types
    //        a fixed length
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is {x}, {y}, {z}");
    println!("The value of z is {}", tup.2); // {tup.2} does not work

    // Array
    // Fixed lengt
    // If need to grow or shrink, use vector
    let a = [1, 2, 3, 4, 5];
    //let b: [i32; 5] = [1, 2, 3, 4, 5]; // create an array of type i32 with 5 elements
    //let c = [3; 5]; // create an array of 5 elements where all elements are initialized with 3.

    println!("a[0]: {} a[1]: {}", a[0], a[1]);
}
