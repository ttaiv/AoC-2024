use std::fs;
use regex::Regex;

pub fn solve() -> (i32, i32) {
    let data = fs::read_to_string("inputs/day3.txt").unwrap();
    // Test data that should give 161 for part 1 and 48 for part 2.
    // let data = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    // Part 1
    let part1_result = find_and_calculate_muls(&data);

    // Begin part 2

    let dont_re = Regex::new(r"don't\(\)").unwrap();
    let do_re = Regex::new(r"do\(\)").unwrap();

    // These hold the offset in the data at which the last found do and dont are.
    let mut current_do: usize = 0; // do is first enabled
    let mut current_dont;

    let mut part2_result = 0;
    loop {
        // Find if there is a dont remaining after last do.
        let dont_match = dont_re.find_at(&data, current_do);
        match dont_match {
            None => {
                // No dont left, just count all the multiplications left.
                part2_result += find_and_calculate_muls(&data[current_do..]);
                break;
            }
            Some(mat) => {
                // There is dont, calculate the multiplications before this.
                current_dont = mat.end();
                part2_result += find_and_calculate_muls(&data[current_do..current_dont]);
            }
        }
        // Find if there is still a do left after this dont.
        let do_mat_option = do_re.find_at(&data, current_dont);
        match do_mat_option {
            None => {
                // No do left, no multiplications left.
                break;
            }
            Some(mat) => {{
                // Update the location of current do and start looking for
                // don't and multiplications again.
                current_do = mat.end();
            }}
        }
    }

    (part1_result, part2_result)
}

fn find_and_calculate_muls(data : &str) -> i32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let result: i32 =  re.captures_iter(&data)
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
