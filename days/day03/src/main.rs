use common::load_vec_from_file;

fn main() {
    // let contents = load_vec_from_file("input/day03/sample.txt").unwrap();
    let contents = load_vec_from_file("input/day03/puzzle.txt").unwrap();
    let mut result = 0;

    for s in contents {
        let len = s.len();
        let l = get_highest_value_as_string(&s[..(len - 1)]);
        let i = s.find(&l[..]).unwrap();
        // dbg!(i);
        let r = get_highest_value_as_string(&s[(i + 1)..]);
        result += (l + &r).parse::<i32>().unwrap();
    }
    println!("{result}");
}

fn get_highest_value_as_string(s: &str) -> String {
    // dbg!(s);
    s.chars()
        .map(|c| c.to_digit(10).unwrap())
        .max()
        .unwrap()
        .to_string()
}
