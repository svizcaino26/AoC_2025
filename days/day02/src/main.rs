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
            let factors = calculate_factors(len);
            for f in factors {
                if is_id_invalid(f, &num_as_str, len) {
                    result += number;
                    break;
                }
            }
        }
    }
    println!("{result}");
}

fn calculate_factors(len: usize) -> Vec<usize> {
    let mut factors = Vec::new();
    let mut i = 1;
    while i * i <= len {
        if len % i == 0 {
            factors.push(i);

            if i * i != len {
                factors.push(len / i);
            }
        }
        i += 1;
    }

    factors.retain(|f| *f != len);
    factors.sort_unstable_by(|a, b| b.cmp(a)); // reverse sort in one

    factors
}

fn is_id_invalid(offset: usize, s: &str, len: usize) -> bool {
    let control = &s[0..offset];

    for i in 0..len / offset {
        if &s[i * offset..(i + 1) * offset] != control {
            return false;
        }
    }

    true
}
