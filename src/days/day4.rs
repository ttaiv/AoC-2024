use crate::utils::Array2D;
use std::usize;

enum Direction {
    North,
    Northeast,
    East,
    Southeast,
    South,
    Southwest,
    West,
    Northwest,
}

pub fn solve() -> (u32, u32) {
    let arr = Array2D::from_file("inputs/day4.txt");

    let part1_result = solve_part1(&arr);
    let part2_result = solve_part2(&arr);

    (part1_result, part2_result)
}

fn solve_part1(arr: &Array2D) -> u32 {
    let result: usize = arr
        .data()
        .iter()
        .enumerate()
        .filter(|(_, &c)| c == 'X') // Find all indices where XMAS can begin
        .map(|(idx, _)| count_part1(idx, arr)) // Count how many XMAS words begins on this index
        .sum();

    result.try_into().unwrap()
}

fn solve_part2(arr: &Array2D) -> u32 {
    let result = arr
        .data()
        .iter()
        .enumerate()
        .filter(|(_, &c)| c == 'A') // Find all indices that have A and thus X-MAS
        .filter(|(idx, _)| is_xmas(*idx, arr)) // Check if it is X-MAS
        .count();

    result.try_into().unwrap()
}

// Count all words "XMAS" starting from this idx that contains char 'X'
fn count_part1(idx: usize, arr: &Array2D) -> usize {
    let x_row = idx / arr.num_rows();
    let x_col = idx % arr.num_rows();

    let x_loc: (i32, i32) = (x_row.try_into().unwrap(), x_col.try_into().unwrap());

    let directions = [
        Direction::East,
        Direction::North,
        Direction::Northeast,
        Direction::Northwest,
        Direction::South,
        Direction::Southeast,
        Direction::Southwest,
        Direction::West,
    ];

    let matches = directions
        .iter()
        .filter(|&dir| is_match_in_dir(&x_loc, arr, dir, "XMAS"))
        .count();

    matches
}

/// Check if we have a match in a certain direction for the given word.
fn is_match_in_dir(x_loc: &(i32, i32), arr: &Array2D, dir: &Direction, word: &str) -> bool {
    let x_loc_converted: (i32, i32) = (x_loc.0.try_into().unwrap(), x_loc.1.try_into().unwrap());

    let calculate_offset = make_offset_calculator(dir);

    for (offset, letter) in word.chars().enumerate() {
        let loc = calculate_offset(x_loc_converted, offset.try_into().unwrap());

        // If loc cannot be converted to usize, it is not a valid index and
        // we cannot have match.
        let loc_usize: (usize, usize) = match (loc.0.try_into(), loc.1.try_into()) {
            (Ok(row), Ok(col)) => (row, col),
            (Err(_), _) | (_, Err(_)) => return false,
        };

        let current = arr.get(loc_usize.0, loc_usize.1);

        if let Some(c) = current {
            if *c == letter {
                // This letter is ok, check next.
                continue;
            }
        }
        // This letter is not ok.
        return false;
    }
    return true;
}

/// Returns true if this idx with letter 'A' has two MASes that form an X
fn is_xmas(idx: usize, arr: &Array2D) -> bool {
    let x_row = idx / arr.num_rows();
    let x_col = idx % arr.num_rows();

    let x_loc: (i32, i32) = (x_row.try_into().unwrap(), x_col.try_into().unwrap());

    // Directions to go from the middle A to look for 'MAS'
    let start_directions = [
        Direction::Northwest,
        Direction::Northeast,
        Direction::Southeast,
        Direction::Southwest,
    ];
    // Directions to look for 'MAS' after going to the starting index.
    let search_directions = [
        Direction::Southeast,
        Direction::Southwest,
        Direction::Northwest,
        Direction::Northeast,
    ];

    let mas_count = start_directions
        .iter()
        .map(|dir| make_offset_calculator(dir)(x_loc, 1)) // Get potential MAS start indices
        .zip(search_directions.iter())
        // Count MASes
        .filter(|(start_loc, dir)| is_match_in_dir(start_loc, arr, dir, "MAS"))
        .count();

    mas_count == 2 // A is the middle of X-MAS if there is exactly two MASes around it
}

/// Return a function that calculates (row, col) from a starting (row, col) to a certain direction
fn make_offset_calculator(dir: &Direction) -> fn((i32, i32), i32) -> (i32, i32) {
    match dir {
        Direction::North => {
            |start_loc: (i32, i32), offset: i32| (start_loc.0 - offset, start_loc.1)
        }
        Direction::Northeast => {
            |start_loc: (i32, i32), offset: i32| (start_loc.0 - offset, start_loc.1 + offset)
        }
        Direction::East => |start_loc: (i32, i32), offset: i32| (start_loc.0, start_loc.1 + offset),
        Direction::Southeast => {
            |start_loc: (i32, i32), offset: i32| (start_loc.0 + offset, start_loc.1 + offset)
        }
        Direction::South => {
            |start_loc: (i32, i32), offset: i32| (start_loc.0 + offset, start_loc.1)
        }
        Direction::Southwest => {
            |start_loc: (i32, i32), offset: i32| (start_loc.0 + offset, start_loc.1 - offset)
        }
        Direction::West => |start_loc: (i32, i32), offset: i32| (start_loc.0, start_loc.1 - offset),
        Direction::Northwest => {
            |start_loc: (i32, i32), offset: i32| (start_loc.0 - offset, start_loc.1 - offset)
        }
    }
}
