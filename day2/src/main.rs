use std::fs;

fn main() {
    let filename = "src/plannedcourse.txt";
    let raw_course = fs::read_to_string(filename).expect("didn't work");
    let course: Vec<&str> = raw_course.split('\n').collect();

    let mut position = Position {
        depth: 0,
        horizontal: 0,
        aim: 0,
    };

    for input in course {
        add_position(input, &mut position);
    }
    println!(
        "depth: {}, distance: {}",
        position.depth, position.horizontal
    );
    println!("{}", position.depth * position.horizontal);
}

struct Position {
    depth: u32,
    horizontal: u32,
    aim: u32,
}

fn add_position(directions: &str, position: &mut Position) {
    let input: Vec<&str> = directions.split(' ').collect();
    let length = input.len();

    if length > 1 {
        let direction = input[0];
        let distance = input[1];

        let distance = match distance.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => 0,
        };

        if direction == "forward" {
            position.horizontal += distance;
            position.depth += position.aim * distance;
        }

        if direction == "down" {
            position.aim += distance;
        }

        if direction == "up" {
            position.aim -= distance;
        }
    } else {
        return;
    }
}
