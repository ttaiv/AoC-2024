use std::fs;
use std::collections::{HashMap, HashSet};
use crate::utils::parse_str_slice_to_integers;

pub fn solve() -> (u32, u32) {
    // Get info about which numbers come after a certain number and all the updates.
    let (successors, page_updates) = parse_input();

    let part1_result: u32 = page_updates.iter()
        .filter(|update| is_in_correct_order(update, &successors))
        .map(|update| update[update.len() / 2])
        .sum::<u32>();

    let part2_result: u32 = page_updates.iter()
        .filter(|update| !is_in_correct_order(update, &successors))
        .map(|incorrect_update| fix_order(incorrect_update, &successors))
        .map(|corrected_update| corrected_update[corrected_update.len() / 2])
        .sum::<u32>();
    
    (part1_result, part2_result)
}

/// Check if update is in correct order.
fn is_in_correct_order(page_update: &Vec<u32>, successors: &HashMap<u32, HashSet<u32>>) -> bool {

    for (page_idx, page_num) in page_update.iter().enumerate() {
        if let Some(successor_list) = successors.get(page_num) {
            // The current page number has numbers that should be after it.
            // Check that no such number has occurred before it.
            for predecessor in page_update[..page_idx].iter() {
                if successor_list.contains(predecessor) {
                    return false;
                }
            }
        }
    }
    return true;
}

/// Return a new update that is the original but correctly ordered
fn fix_order(page_update: &Vec<u32>, successors: &HashMap<u32, HashSet<u32>>) -> Vec<u32> {
    // Use dfs to topologically sort the update.
    let mut dfs_visited_order = subgraph_dfs(successors, page_update);
    dfs_visited_order.reverse();
    dfs_visited_order
}

// Perform dfs for the subgraph consisting only some nodes (nodes_to_consider)
fn subgraph_dfs(adj_list: &HashMap<u32, HashSet<u32>>, nodes_to_consider: &[u32]) -> Vec<u32> {

    let mut visited: HashSet<u32> = HashSet::new();
    let mut finished_order: Vec<u32> = Vec::new();

    fn dfs(current_node: u32, adj_list: &HashMap<u32, HashSet<u32>>, nodes_to_consider: &[u32],
        visited: &mut HashSet<u32>, finished_order: &mut Vec<u32>) {
        
        visited.insert(current_node);

        let neighbors = adj_list.get(&current_node);

        if let Some(neighbor_list) = neighbors {
            for &neighbor in neighbor_list {
                if nodes_to_consider.contains(&neighbor) && !visited.contains(&neighbor) {
                    dfs(neighbor, adj_list, nodes_to_consider, visited, finished_order)
                }
            }
        }
        finished_order.push(current_node);
    }

    for &node in nodes_to_consider {
        if !visited.contains(&node) {
            dfs(node, adj_list, nodes_to_consider, &mut visited, &mut finished_order);
        }
    }
    finished_order
}

fn parse_input() -> (HashMap<u32, HashSet<u32>>, Vec<Vec<u32>>) {
    let data = fs::read_to_string("inputs/day5.txt")
        .expect("Input file should be available");

    let mut successors: HashMap<u32, HashSet<u32>> = HashMap::new();

    data.lines()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let mut nums = line.split('|');
            let num1: u32 = nums.next().unwrap().parse().unwrap();
            let num2: u32 = nums.next().unwrap().parse().unwrap();
            (num1, num2)
        })
        .for_each(|(num1, num2)| {
            // num2 must be after num1
            let successor_list = successors.entry(num1).or_insert(HashSet::new());
            successor_list.insert(num2);
        });

    // Separator that separates rules and updates
    let sep = data.find("\n\n")
        .expect("The input should contain two line breaks separating the rules and updates");

    let update_data = &data[sep + 2..];
    let updates = parse_str_slice_to_integers(update_data, ",");

    (successors, updates)
}
