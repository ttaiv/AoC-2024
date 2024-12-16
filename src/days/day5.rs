use std::fs;
use std::collections::{HashMap, HashSet};
use crate::utils::parse_str_slice_to_integers;

pub fn solve() -> (u32, u32) {

    let (must_precede, page_updates) = parse_input();

    

    (0, 0)
}

fn parse_input() -> (HashMap<u32, HashSet<u32>>, Vec<Vec<u32>>) {
    let data = fs::read_to_string("inputs/day5_test.txt")
        .expect("Input file should be available");

    let mut must_precede: HashMap<u32, HashSet<u32>> = HashMap::new();

    data.lines()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let mut nums = line.split('|');
            let num1: u32 = nums.next().unwrap().parse().unwrap();
            let num2: u32 = nums.next().unwrap().parse().unwrap();
            (num1, num2)
        })
        .for_each(|(num1, num2)| {
            // num1 must precede num2
            let predecessors = must_precede.entry(num2).or_insert(HashSet::new());
            predecessors.insert(num1);
        });

    // Separator that separates rules and updates
    let sep = data.find("\n\n")
        .expect("The input should contain two line breaks separating the rules and updates");

    let page_updates = &data[sep + 2..];

    let updates = parse_str_slice_to_integers(page_updates, ",");

    (must_precede, updates)
}
