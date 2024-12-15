use std::fs;

/// Parses the text file for given day to a vector of vectors of integers.
pub fn parse_input(day_num: u32) -> Vec<Vec<i32>> {
    let input_path = format!("inputs/day{day_num}.txt");

    let contents = fs::read_to_string(input_path)
        .expect("Input file should be available");

    let input_arr: Vec<Vec<i32>> = contents
        .lines()
        .map(|line| {
            line.split_whitespace()
            .map(|num| num.parse::<i32>()
            .expect("Rows should contain valid numbers"))
            .collect()
        })
        .collect();

    return input_arr;
}

#[derive(Debug)]
pub struct Array2D {
    data: Vec<char>,
    num_rows: usize,
    num_cols: usize,
}

impl Array2D {
    pub fn data(&self) -> &[char] {
        &self.data
    }

    pub fn num_rows(&self) -> usize {
        self.num_rows
    }

    pub fn num_cols(&self) -> usize {
        self.num_cols
    }

    pub fn from_file(file_path: &str) -> Self {
        let contents = fs::read_to_string(file_path)
            .expect("Input file should be available");

        let rows: Vec<&str> = contents.lines().collect();

        let num_rows = rows.len();
        let num_cols = rows.get(0).map_or(0, |row| row.chars().count());

        let data: Vec<char> = rows.iter().flat_map(|&row| row.chars()).collect();

        Self {
            data,
            num_rows,
            num_cols,
        }
    }

    pub fn get(&self, row: usize, col: usize) -> Option<&char> {
        if row < self.num_rows() && col < self.num_cols() {
            Some(&self.data[row * self.num_cols + col])
        } else {
            None
        }
    }
}
