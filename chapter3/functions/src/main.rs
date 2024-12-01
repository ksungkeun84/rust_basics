// Parameters must be type-annotated
fn another_function(x: i32) {
    println!("Te value of x is {x}");
}

fn five() -> i32 {
    5
}

fn main() {
    another_function(5);

    let x = five();
    println!("The value of x is {x}");
}
