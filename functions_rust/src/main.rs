// fn main() {
//     // another_function(5);
//     print_labeled_measurement(5, 'h');
//     println!("Done");
// }
// // fn another_function(x: i32) {
// //     println!("The value of x is: {x}")
// // }

// fn print_labeled_measurement(value: i32, unit_label: char) {
//     println!("The measurement is: {value}{unit_label}");
// }
// fn main() {
//     let mut count: u32 = 0;
//     'counting_up: loop {
//         println!("count = {count}");
//         let mut remaining: u32 = 10;

//         loop {
//             println!("remaining = {remaining}");
//             if remaining == 9 {
//                 break;
//             }
//             if count == 2 {
//                 break 'counting_up;
//             }
//             remaining -= 1;
//         }
//         count += 1;
//     }
//     println!("end of the count = {count}");
// }
// Referencing example
// fn main() {
//     let s1 = String::from("hello");
//     let len = calculate_length(&s1);
//     println!("the length of {} is {}. ", s1, len);
// }
// fn calculate_length(s: &String) -> usize {
//     s.len()
// }

fn main() {
    let mut s = String::from("Hello");
    change(&mut s);
    println!("change mutable string: {}", s)
}
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
