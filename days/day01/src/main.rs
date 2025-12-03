use common::load_vec_from_file;

fn main() {
    let rotations =
        load_vec_from_file("input/day01.txt").unwrap_or_else(|e| panic!("Error loading file: {e}"));

    let mut position = 50;
    let mut password = 0;

    for r in rotations {
        let mut parts = r.chars();
        let direction = parts.next().unwrap();
        let clicks: i32 = parts.collect::<String>().parse().unwrap();
        rotate_dial(&mut position, &mut password, clicks, &direction);
    }
    println!("The password is: {password}");
}

fn rotate_dial(position: &mut i32, password: &mut i32, clicks: i32, direction: &char) {
    match direction {
        'R' => {
            *position += clicks;
            while *position > 99 {
                *position -= 100;
            }
            if *position == 0 {
                *password += 1;
            }
            return;
        }
        'L' => {
            *position -= clicks;
            while *position < 0 {
                *position += 100;
            }
            if *position == 0 {
                *password += 1;
            }
            return;
        }
        _ => (),
    }
}
