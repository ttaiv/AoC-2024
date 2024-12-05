use crate::utils;

pub fn solve() -> (u32, u32) {
    let reports = utils::parse_input(2);

    let safe_count: u32 = 
        reports.iter()
        .filter(|&report| is_safe(&report))
        .count()
        .try_into()
        .expect("usize should be converted to u32");

    let safe_count_part2: u32 = 
        reports.iter()
        .filter(|&report| is_safe_with_problem_dampener(&report))
        .count()
        .try_into()
        .expect("usize should be converted to u32");

    (safe_count, safe_count_part2)
}

fn is_safe(report: &[i32]) -> bool {
    if report.len() < 2 {
        // Report with one or zero levels is always safe.
        return true;
    }

    let decreasing = report[0] > report[1];

    for adj_levels in report.windows(2) {
        let diff = 
            if decreasing {
                adj_levels[0] - adj_levels[1]
            } else {
                adj_levels[1] - adj_levels[0]
            };

        if diff < 1 || diff > 3 {
            return false;
        }
    }
    true
}

fn is_safe_with_problem_dampener(report: &Vec<i32>) -> bool {
    if report.len() < 2 {
        // Report with one or zero levels is always safe.
        return true;
    }

    // Test if all levels are decreasing with at most one skip.
    if is_safe_with_skip(report, true) {
        return true;
    }
    // Test if all levels are increasing with at most one skip.
    if is_safe_with_skip(report, false) {
        return true;
    }
    // Last option is to skip the first level.
    return is_safe(&report[1..]);

    fn is_safe_with_skip(report: &Vec<i32>, decreasing: bool) -> bool {
        let mut it_right = report.iter();
        let mut level_left = it_right.next().unwrap();
        let mut skip_available = true;
    
        while let Some(level_right) = it_right.next() {
            let diff = {
                if decreasing {
                    level_left - level_right
                } else {
                    level_right - level_left
                }
            };
        
            if diff < 1 || diff > 3 {
                if skip_available {
                    skip_available = false;
                    // Move the right iterator but keep the left level the same.
                    continue;
                }
                // fail
                return false;
            }
            level_left = level_right;
        }
        true
    }


    /*
    let failing_indices = get_failing_indices(report, None);

    match failing_indices {
        None => {
            return true;
        }
        Some((left, right)) => {
            let failing2 = get_failing_indices(report, Some(left));
            if failing2.is_none() {
                return true;
            }
            let failing3 = get_failing_indices(report, Some(right));
            if failing3.is_none() {
                return true;
            }
            return false;
        }
    }

    fn get_failing_indices(report: &Vec<i32>, exclude_idx: Option<usize>) -> Option<(usize, usize)> {
        
        let (start_idx_right, skip_idx) = match exclude_idx {
            Some(0) => (2, None),
            Some(not_zero) => (1, Some(not_zero)),
            None => (1, None)
        };

        let mut idx_left = start_idx_right - 1;
        let decreasing = report[idx_left] > report[start_idx_right];
        
        for (idx_right, &level_right) in report.iter().enumerate() {
            if idx_right < start_idx_right {
                idx_left = idx_right;
                continue;
            }

            match skip_idx {
                Some(idx) => {
                    if idx == idx_right {
                        continue;
                    }
                }
                None => ()
            }

            let level_left = report[idx_left];

            let diff = {
                if decreasing {
                    level_left - level_right
                } else {
                    level_right - level_left
                }
            };

            if diff < 1 || diff > 3 {
                return Some((idx_left, idx_right));
            }

            idx_left = idx_right;
        }
        None
    }

    */

}
