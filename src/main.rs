use std::env;
use aoc::days;

fn main() {

    let day_to_solve: u32 = env::args().nth(1)
        .expect("Day number should be given as argument")
        .parse()
        .expect("Day should be a number");

    match day_to_solve {
        1 => {
            let (total_dist, similarity_score) =  days::day1::solve();
            println!("Total distance between location ID lists: {}", total_dist);
            println!("Similarity score: {similarity_score}");
        }
        2 => {
            let safe_counts = days::day2::solve();
            println!("Safe report without problem dampener: {}", safe_counts.0);
            println!("Safe reports with problem dampener: {}", safe_counts.1);
        }
        3 => {
            let result = days::day3::solve();
            println!("Sum of multiplications in part 1 is {}", result.0);
            println!("Sum of multiplications in part 2 {}", result.1);
        }
        4 => {
            let result = days::day4::solve();
            default_print_results(result);
        }
        5 => {
            default_print_results(days::day5::solve());
        }
        _ => {
            println!("Day {day_to_solve} not implemented yet!");
        }
    }

}

fn default_print_results(result: (u32, u32)) {
    println!("Part 1 result: {}", result.0);
    println!("Part 2 result: {}", result.1);
}