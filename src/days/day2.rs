use crate::utils;

pub fn solve() -> (u32, u32) {
    let reports = utils::parse_input(2);

    let safe_count_part1: u32 = reports
        .iter()
        .filter(|&report| is_safe(&report))
        .count()
        .try_into()
        .expect("usize should be converted to u32");

    let safe_count_part2: u32 = reports
        .iter()
        .filter(|&report| is_safe_with_problem_dampener(&report))
        .count()
        .try_into()
        .expect("usize should be converted to u32");

    (safe_count_part1, safe_count_part2)
}

fn is_safe(report: &[i32]) -> bool {
    if report.len() < 2 {
        // Report with one or zero levels is always safe.
        return true;
    }

    let decreasing = report[0] > report[1];

    for adj_levels in report.windows(2) {
        let diff = if decreasing {
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

    if is_safe(report) {
        return true;
    }

    // Try if the report if safe if one level is ignored.
    for skip_idx in 0..report.len() {
        let before_skip = &report[..skip_idx];
        let after_skip = &report[skip_idx + 1..];
        let modified_report: Vec<i32> = before_skip
            .iter()
            .chain(after_skip.iter())
            .cloned()
            .collect();

        if is_safe(&modified_report) {
            return true;
        }
    }
    false
}

// This misses three reports that should be safe
// [26, 25, 22, 24, 23]
// [66, 68, 67, 68, 70]
// [32, 35, 33, 34, 35, 38]
#[allow(dead_code)]
fn is_safe_with_problem_dampener2(report: &Vec<i32>) -> bool {
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
}
