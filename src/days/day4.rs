use std::usize;
use crate::utils::Array2D;

enum Direction {
    North,
    Northeast,
    East,
    Southeast,
    South,
    Southwest,
    West,
    Northwest
}

pub fn solve() -> u32 {
    let arr = Array2D::from_file("inputs/day4.txt");

    // println!("{arr:?}");

    let result: usize = arr.data().iter().enumerate()
        .filter(|(_, &c)| c == 'X')
        .map(|(idx, _)| count_xmas(idx, &arr))
        .sum();

    result.try_into().unwrap()
}

// Count all words "XMAS" starting from this idx that contains char 'X'
fn count_xmas(idx: usize, arr: &Array2D) -> usize {
    let x_row = idx / arr.num_rows();
    let x_col = idx % arr.num_rows();

    let x_loc = (x_row, x_col);

    let directions = [
        Direction::East,
        Direction::North,
        Direction::Northeast,
        Direction::Northwest,
        Direction::South,
        Direction::Southeast,
        Direction::Southwest,
        Direction::West
    ];
    
    let matches = directions.iter()
        .filter(|&dir| is_match_in_dir(x_loc, arr, dir))
        .count();

    matches
}

/// Check if we have a match in a certain direction given that we have 'X' at x_loc.
fn is_match_in_dir(x_loc: (usize, usize), arr: &Array2D, dir: &Direction) -> bool {
    let letters = ['M', 'A', 'S'];
    let x_loc_converted: (i32, i32) = (x_loc.0.try_into().unwrap(), x_loc.1.try_into().unwrap());

    let calculate_offset = make_offset_calculator(dir);

    for (idx, letter) in letters.iter().enumerate() {

        let offset: i32 = (idx + 1).try_into().unwrap();

        let loc = calculate_offset(x_loc_converted, offset);

        // If loc cannot be converted to usize, it is not a valid index and
        // we cannot have match.
        let loc_usize: (usize, usize) = match (loc.0.try_into(), loc.1.try_into()) {
            (Ok(row), Ok(col)) => (row, col),
            (Err(_), _) | (_, Err(_)) => return false
        };

        let current = arr.get(loc_usize.0, loc_usize.1);

        if let Some(c) = current {
            if c == letter {
                // This letter is ok, check next.
                continue;
            }
        }
        // This letter is not ok.
        return false;
    }
    return true;
}

/// Return a function that calculates (row, col) from a starting (row, col) to a certain direction
fn make_offset_calculator(dir: &Direction) -> fn((i32, i32), i32) -> (i32, i32) {
    match dir {
        Direction::North => |start_loc: (i32, i32), offset: i32| (start_loc.0 - offset, start_loc.1),
        Direction::Northeast => |start_loc: (i32, i32), offset: i32| (start_loc.0 - offset, start_loc.1 + offset),
        Direction::East => |start_loc: (i32, i32), offset: i32| (start_loc.0, start_loc.1 + offset),
        Direction::Southeast => |start_loc: (i32, i32), offset: i32| (start_loc.0 + offset, start_loc.1 + offset),
        Direction::South => |start_loc: (i32, i32), offset: i32| (start_loc.0 + offset, start_loc.1),
        Direction::Southwest => |start_loc: (i32, i32), offset: i32| (start_loc.0 + offset, start_loc.1 - offset),
        Direction::West => |start_loc: (i32, i32), offset: i32| (start_loc.0, start_loc.1 - offset),
        Direction::Northwest => |start_loc: (i32, i32), offset: i32| (start_loc.0 - offset, start_loc.1 - offset),
    }
}
