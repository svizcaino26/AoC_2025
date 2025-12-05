use common::load_vec_from_file;

fn main() {
    let rotations =
        load_vec_from_file("input/day01.txt").unwrap_or_else(|e| panic!("Error loading file: {e}"));

    let mut position = 50;
    let mut password = 0;

    for r in rotations {
        let mut parts = r.chars();
        let direction = parts.next().unwrap();
        let mut clicks: i32 = parts.collect::<String>().parse().unwrap();
        if clicks > 100 {
            perform_cycles(&mut clicks, &mut password);
        }
        rotate_dial(&mut position, &mut password, clicks, &direction);
    }
    println!("The password is: {password}");
}

fn perform_cycles(clicks: &mut i32, password: &mut i32) {
    *password += *clicks / 100;
    *clicks = *clicks % 100;
}

fn rotate_dial(position: &mut i32, password: &mut i32, clicks: i32, direction: &char) {
    let old = *position;
    match direction {
        'R' => {
            *position += clicks;
            if *position > 99 {
                *password += 1;
                *position -= 100;
                return;
            }
        }
        'L' => {
            *position -= clicks;
            if *position < 0 {
                if old != 0 {
                    *password += 1;
                }
                *position += 100;
                return;
            }
        }
        _ => (),
    }

    if *position == 0 {
        *password += 1;
    }
}
