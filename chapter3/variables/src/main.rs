// Differences bet. Variables and Constants
// Constant
// - can't be mutable.
// - must be declared with type annotation
// - can be declared in any scope including the global scope
// - may be set only to a constant expression, not the result of a value that could only be computed
//   at runtime
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn shadowing() {
    // Shadowing is different from marking a variable as `mut` because we'll get a compile-time
    // error if we accidentally try to reassign to this variable without using the `let` keyword.
    // By using `let`, we can perform a few transformations on a value but have the variable be
    // immutable after those transformations have been completed.
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is {x}");
    }
    println!("The value of x is {x}");
}

fn main() {
    // A variable is immutable by default so to make it mutable,
    // mut keywork should be placed in front of the variable names.
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    shadowing();
}
