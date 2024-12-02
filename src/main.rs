use std::fs;

fn main() {
    let day1_input_path = "inputs/day1.txt";

    let day1_input = fs::read_to_string(day1_input_path)
        .expect("File should be read");

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

    println!("Total distance between location ID lists: {}", total_dist);
}
