use std::env;
mod days;
mod utils;

fn main() {

    let day_to_solve: u32 = env::args().nth(1)
        .expect("Day number should be given as argument")
        .parse()
        .expect("Day should be a number");

    match day_to_solve {
        1 => {
            let (total_dist, similarity_score) =  days::day1::solve();
            println!("\nDay 1 solutions");
            println!("----------------------------");
            println!("Total distance between location ID lists: {}", total_dist);
            println!("Similarity score: {similarity_score}");
        }
        2 => {
            let safe_count = days::day2::solve();
            println!("Safe report count: {safe_count}");
        }
        _ => {
            println!("Day {day_to_solve} not implemented yet!");
        }
    }

}