use std::{fs::read_to_string, collections::HashMap};

#[allow(dead_code)]
const NUM_STRINGS:[&'static str; 9] = ["one","two", "three","four","five","six","seven","eight","nine"];

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
// TASK: WRONG needs to replace first valid string not every string that matches
// it can have this eightwo or oneight
fn part2(filename: &str) -> i32 {
    let mut end_vec: Vec<i32> = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        let mut tmp = String::from(line);
        let mut backwards = 9;
        for _n in 1..10 {
            tmp = tmp.replace(NUM_STRINGS[backwards-1],&backwards.to_string());
            backwards-=1;
        }
        let split_to_char: Vec<char> = tmp.chars().collect();
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

fn part_2(input: &str) -> u32 {
    let mut nums = HashMap::new();
    nums.insert("1", 1);
    nums.insert("2", 2);
    nums.insert("3", 3);
    nums.insert("4", 4);
    nums.insert("5", 5);
    nums.insert("6", 6);
    nums.insert("7", 7);
    nums.insert("8", 8);
    nums.insert("9", 9);
    nums.insert("one", 1);
    nums.insert("two", 2);
    nums.insert("three", 3);
    nums.insert("four", 4);
    nums.insert("five", 5);
    nums.insert("six", 6);
    nums.insert("seven", 7);
    nums.insert("eight", 8);
    nums.insert("nine", 9);

    let mut sum = 0;
    for line in input.lines() {
        let mut forwards = line;
        let mut backwards = line;
        let first = 'outer: loop {
            for (prefix, num) in nums.iter() {
                if forwards.starts_with(prefix) {
                    break 'outer num;
                }
            }
            forwards = &forwards[1..];
        };
        let last = 'outer: loop {
            for (suffix, num) in nums.iter() {
                if backwards.ends_with(suffix) {
                    break 'outer num;
                }
            }
            backwards = &backwards[..backwards.len() - 1];
        };
        let num = first * 10 + last;
        sum += num;
    }
    sum
}

fn main() {
    // let args: Vec<String> = env::args().collect();
    // // cargo run -q input.txt
    // let _arg_filename = &args[1];
    let file_name = "input.txt";
    println!("Mine code: {:?}", part1(file_name));
    let input = include_str!("../input.txt");
    println!("Using iterarator chain: {:?}", part_1(input));
    println!("Mine code part2: {:?}", part2(file_name));
    println!("Using iterarator chain: {:?}", part_2(input));
}
