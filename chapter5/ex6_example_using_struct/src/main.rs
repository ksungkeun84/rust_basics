
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {

    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),        // dbg! prints info and returns ownership
        height: 50,
    };

    println!("The area of the rectangle is {} square pixels.",
        area(&rect1));
    println!("rect1 is {:?}", rect1);       // {:#?} is another option
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
