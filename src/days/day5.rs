use std::fs;
use std::collections::{HashMap, HashSet};
use crate::utils::parse_str_slice_to_integers;

pub fn solve() -> (u32, u32) {

    let (must_precede, page_updates) = parse_input();

    let part1_result: u32 = page_updates.iter()
        .filter(|update| is_in_correct_order(update, &must_precede))
        .map(|update| update[update.len() / 2])
        .sum::<u32>();

    let incorrectly_ordered: Vec<Vec<u32>> = page_updates.iter()
        .filter(|update| !is_in_correct_order(update, &must_precede))
        .cloned()
        .collect();

    

    
    (part1_result, 0)
}

fn is_in_correct_order(page_update: &Vec<u32>, must_precede: &HashMap<u32, HashSet<u32>>) -> bool {

    for (page_idx, page_num) in page_update.iter().enumerate() {

        for prev_page in page_update[..page_idx].iter() {
            if let Some(predecessors) = must_precede.get(&prev_page) {
                // There are number(s) that must be before the previous page.
                // Check that the current one is not one of them.
                if predecessors.contains(page_num) {
                    return false;
                }
            }
        }   
    }
    return true;
}

/*
fn fix_order(page_update: &Vec<u32>, must_precede: &HashMap<u32, HashSet<u32>>) -> Vec<u32> {

    let mut corrected_update = page_update.clone();

    for (page_idx, page_num) in page_update.iter().enumerate() {

        for prev_page in pages_in_update {
            if let Some(predecessors) = must_precede.get(&prev_page) {
                // There are number(s) that must be before the previous page.
                // Check if the current one is one of them.
                if predecessors.contains(page_num) {
                    
                }
            }
        }
        pages_in_update[page_idx] = *page_num;        
    }

    corrected_update
    
}
    */

fn parse_input() -> (HashMap<u32, HashSet<u32>>, Vec<Vec<u32>>) {
    let data = fs::read_to_string("inputs/day5.txt")
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
