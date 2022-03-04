struct Rectangle {
    width: u32,
    height: u32,
}
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50
    };
    println!(
        "The area of rectangle is {}",
        area(&rect1)
    );
}
fn area(rectangle: &Rectangle) -> u32 {
    reactangle.width * rectangle.height
}