use std::fs;
use std::collections::HashMap;

pub fn solve() -> (i32, i32) {
    let day1_input_path = "inputs/day1.txt";

    let day1_input = fs::read_to_string(day1_input_path)
        .expect("File should be read");

    // Begin part 1

    let mut list1 = Vec::new();
    let mut list2 = Vec::new();

    for line in day1_input.lines() {
        let mut parts = line.split_whitespace();

        let first: i32 = parts.next().expect("Line should have string")
            .parse().expect("String should be an integer");

        let second: i32 = parts.next().expect("Line should have string")
            .parse().expect("String should be an integer");

        list1.push(first);
        list2.push(second);
    }

    list1.sort_unstable();
    list2.sort_unstable();

    let distances: Vec<i32> = list1.iter().zip(list2.iter())
        .map(|(id1, id2)| (id1 - id2).abs())
        .collect();

    let total_dist: i32 = distances.iter().sum();

    // Begin part 2

    let mut right_list_id_counts = HashMap::new();

    for id in list2 {
        let count = right_list_id_counts.entry(id).or_insert(0);
        *count += 1;
    }

    let mut similarity_score = 0;
    for id in list1 {
        let right_count = right_list_id_counts.get(&id).unwrap_or(&0);
        similarity_score += right_count * id;
    }

    return (total_dist, similarity_score)
}
