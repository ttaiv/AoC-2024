use std::fs;
use regex::Regex;

pub fn solve() -> i32 {
    // Read program memory data.
    let data = fs::read_to_string("inputs/day3.txt").unwrap();
    // Form regex to search for pattern mul(num1,num2)
    let multiply_re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let result: i32 =  multiply_re.captures_iter(&data)
        .map(|caps| {
        // Make pairs of numbers to be multiplied.
        let num1: i32 = caps[1].parse().unwrap();
        let num2: i32 = caps[2].parse().unwrap();
        (num1, num2)
        })
        .map(|(num1, num2)| num1 * num2) // Do the multiplication
        .sum();

    result
}