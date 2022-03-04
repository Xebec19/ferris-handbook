#[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }
// fn main() {
//     let rect1 = Rectangle {
//         width: dbg!(32),
//         height: 32
//     };
//     println!(
//         "The area of rectangle is {:#?}",
//         rect1
//     );
// }
struct Reactangle {
    width: u32,
    height: u32,
}

impl Reactangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let react1 = Reactangle{
        width: 30,
        height: 50,
    };
    println!(
        "The area of the reactangle is {} square pixels.",
        react1.area()
    );
}