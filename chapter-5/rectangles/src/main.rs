
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

fn main() {
    let rectangle1 = Rectangle {
        width: 30,
        height: 50
    };

    println!(
        "The area of the rectangle is: {}",
        area(&rectangle1)
    );

    println!(
        "rectangle1: {:#?}",
        rectangle1
    )
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}