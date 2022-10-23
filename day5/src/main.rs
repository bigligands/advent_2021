use std::fs;

fn main() {
    let path = "input.txt";
    let mut raw_input = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(_) => panic!("path not found"),
    };
    parse_raw_input(&mut raw_input);
}

fn parse_raw_input(input: &str) {
    // want to take input and collect x & y points
    // turn points into lines
    // check for vertical / horizontal
    // collect points along line
    // compare collection for overlap -- set?
    // how many points have at least 2 overlaps?
    let split_raw_input = input.split('\n').collect::<Vec<&str>>();
    for line in split_raw_input {}
}

struct Point {
    x: u16,
    y: u16,
}
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn get_pts_in_line(&self) -> Option<Vec<u16>> {
        if self.start.x == self.end.x {
            println!("vertical line detected");
            let pts_in_line = (self.start.y..=self.end.y).collect::<Vec<u16>>();
            return Some(pts_in_line);
        } else if self.start.y == self.end.y {
            println!("horizontal line detected");
            let pts_in_line = (self.start.x..=self.end.x).collect::<Vec<u16>>();
            return Some(pts_in_line);
        }
        None
    }
}
