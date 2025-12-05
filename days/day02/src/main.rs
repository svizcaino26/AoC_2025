use std::fs;

fn main() {
    let contents = fs::read_to_string("input/day02.txt").expect("Failed to read file");
    let contents: Vec<&str> = contents.trim().split(",").collect();
    let mut result: u64 = 0;

    for pair in contents {
        let bounds: Vec<&str> = pair.split("-").collect();
        let lower_bound: u64 = bounds[0].parse().unwrap();
        let upper_bound: u64 = bounds[1].parse().unwrap();
        for number in lower_bound..=upper_bound {
            let num_as_str = number.to_string();
            let len = num_as_str.len();
            if len.is_multiple_of(2) {
                let (left, right) = num_as_str.split_at(len / 2);
                if left == right {
                    result += number;
                }
            }
        }
    }
    println!("{result}");
}
