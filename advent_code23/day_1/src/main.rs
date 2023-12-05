use std::fs::read_to_string;

#[allow(dead_code)]
const NUM_STRINGS:[&'static str; 3] = ["one","two", "three"];

fn part1(filename: &str) -> i32 {
    let mut end_vec: Vec<i32> = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        let split_to_char: Vec<char> = line.chars().collect();
        let mut line_vec: Vec<i32> = Vec::new();
        for chars in split_to_char {
            if '0' <= chars && chars <= '9' {
                line_vec.push(chars as i32 - '0' as i32);
            }
        }
        let fist_last: i32 = line_vec[0] * 10 + line_vec[line_vec.len() - 1];
        end_vec.push(fist_last)
    }
    // sum of vector
    let result: i32 = end_vec.iter().sum();
    result
}

fn part_1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let first = line.chars().find_map(|c| c.to_digit(10)).unwrap();
            let last = line.chars().rev().find_map(|c| c.to_digit(10)).unwrap();
            10 * first + last
        })
        .sum()
}

fn main() {
    // let args: Vec<String> = env::args().collect();
    // // cargo run -q input.txt
    // let _arg_filename = &args[1];
    let file_name = "input.txt";
    println!("Mine code: {:?}", part1(file_name));
    let input = include_str!("../input.txt");
    println!("Using iterarator chain: {:?}", part_1(input));
}