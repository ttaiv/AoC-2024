use std::fs;

pub fn parse_input(day_num: u32) -> Vec<Vec<i32>> {
    let input_path = format!("inputs/day{day_num}.txt");

    let contents = fs::read_to_string(input_path)
        .expect("Input file should be available");

    let input_arr: Vec<Vec<i32>> = contents
        .lines()
        .map(|line| {
            line.split_whitespace()
            .map(|num| num.parse::<i32>()
            .expect("Rows should contain valid numbers"))
            .collect()
        })
        .collect();

    return input_arr;
}