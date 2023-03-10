#![allow(unused_variables)]
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

// fn main() {
//     let mut s = String::from("Hello");
//     change(&mut s);
//     println!("change mutable string: {}", s)
// }
// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }

// fn main() {
//     let string = no_dangle();
// }
// fn no_dangle() -> String {
//     let s = String::from("hello");
//     s
// }

fn first_word(s: &str) -> &str {
    let bytes: &[u8] = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn main() {
    let my_string = String::from("hello world sadd");
    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word(&my_string[0..6]);
    println!("Length of string is :{}", word.len());
    let word = first_word(&my_string[..]); // same as above variable _word
                                           // println!("the first word is {}", _word);
                                           // `first_word` also works on references to `String`s, which are equivalent
                                           // to whole slices of `String`s
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
    println!("the first word is {}", word);
}
