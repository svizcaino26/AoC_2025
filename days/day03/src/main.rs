use std::collections::HashSet;

use common::load_vec_from_file;

fn main() {
    // let contents = load_vec_from_file("input/day03/sample.txt").unwrap();
    let contents = load_vec_from_file("input/day03/puzzle.txt").unwrap();
    let mut result = 0;
    let mut selected_batteries = String::new();
    let mut cursor: usize;
    let mut remaining_batteries;
    let mut remaining_positions;
    let mut digits: Vec<u32> = Vec::new();

    for s in contents {
        selected_batteries.clear();
        cursor = 0;
        remaining_positions = 12;
        remaining_batteries = &s[cursor..];
        digits.clear();

        while remaining_positions > 0 {
            remaining_batteries = &remaining_batteries[cursor..];
            remaining_positions -= 1;
            digits = extract_unique_digits(&s[cursor..]);
            digits.sort_unstable_by(|a, b| b.cmp(a));

            for d in &digits {
                if let Some(i) = get_highest_value_as_string(
                    remaining_batteries,
                    remaining_positions,
                    &d.to_string(),
                ) {
                    selected_batteries += &remaining_batteries[i..=i];
                    cursor = i + 1;
                    break;
                } else {
                    continue;
                }
            }
        }
        result += selected_batteries.parse::<u64>().unwrap();
    }
    println!("Result: {result}");
}

fn get_highest_value_as_string(s: &str, remaining: usize, num: &str) -> Option<usize> {
    let len = s.len();
    s.chars()
        .position(|c| c.to_string() == num)
        .take_if(|p| (len - 1) - *p >= remaining)
}

fn extract_unique_digits(s: &str) -> Vec<u32> {
    s.chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<HashSet<_>>()
        .into_iter()
        .collect()
}
