#![allow(unused)]
// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }

// fn main() {
//     let user1 = User {
//         email: String::from("someone@example.com"),
//         username: String::from("Steppenlion"),
//         active: true,
//         sign_in_count: 2,
//     };
// }

// fn main() {
//     let width1 = 30;
//     let height1 = 50;

//     println!(
//         "The area of rectangle is {} square pixels.",
//         area(width1, height1)
//     );
// }

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// REFACTORING USING TUPLES

// fn main() {
//     let rect1 = (30, 50);

//     println!("The area of rectangle is {} square pixels.", area(rect1));
// }
// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// Refactoring with Structs: Adding More Meaning

// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };
//     println!("the Area of rectangle is {}", area(&rect1));
// }

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }

// Adding Useful Functionality with Derived Traits
// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };
//     // tree ways to see debug lines:
//     println!("rect1 is {:?}\n", rect1);
//     println!("rect1 is {:#?}'\n", rect1);
//     dbg!(&rect1);
// }

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
fn main() {
    let sq = Rectangle::square(3);
    println!("Square is : {:?}", sq);
}
