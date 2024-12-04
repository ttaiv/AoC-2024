use crate::utils;

pub fn solve() -> u32 {
    let reports = utils::parse_input(2);

    let safe_count: u32 = 
        reports.iter()
        .filter(|&report| is_safe(&report))
        .count()
        .try_into()
        .expect("usize should be converted to u32");

    return safe_count;
}

fn is_safe(report: &Vec<i32>) -> bool {
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
    return true;
}