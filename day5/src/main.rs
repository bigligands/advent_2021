use std::{collections::HashSet, fs};

fn main() {
    let path = "src/input.txt";
    let mut raw_input = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(_) => panic!("path not found"),
    };
    parse_raw_input(&mut raw_input);
}

fn parse_raw_input(input: &str) {
    // want to take input and collect x & y points
    // turn points into lines - complete
    // check for vertical / horizontal - complete
    // collect points along line - complete
    // compare collection for overlap -- set?
    // how many points have at least 2 overlaps?
    let split_raw_input = input.split('\n').collect::<Vec<&str>>();
    let mut line_collection: Vec<Line> = Vec::new();
    for line in split_raw_input {
        // 1,2 -> 3,4
        // 1,2,3,4
        let mut trimmed_line = line.replace(&"->", &",").clone();
        trimmed_line.retain(|x| x != ' ');
        // println!("{}", trimmed_line);
        let number_strings = trimmed_line.split(',').collect::<Vec<&str>>();
        // last value end with /r
        // println!("{:?}", number_strings);
        let mut numbers: Vec<u16> = Vec::new();
        'sloop: for s in number_strings {
            if s.is_empty() {
                continue 'sloop;
            }

            let conversion = match u16::from_str_radix(s.trim(), 10) {
                Ok(x) => x,
                Err(_) => panic!("Failed to convert str to u16: {}", s),
            };
            numbers.push(conversion);
        }
        if numbers.len() > 0 {
            line_collection.push(create_line(numbers));
        }
    }

    let mut point_set: HashSet<Point> = HashSet::new();
    let mut repeated_values: HashSet<Point> = HashSet::new();

    for line in line_collection {
        if let Some(points) = line.get_pts_in_line() {
            for point in points {
                if !point_set.insert(point) {
                    repeated_values.insert(point);
                }
            }
        }
    }
    println!("{}", repeated_values.len());
}

#[derive(Eq, Hash, PartialEq, Copy, Clone)]
struct Point {
    x: u16,
    y: u16,
}
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn get_pts_in_line(&self) -> Option<Vec<Point>> {
        if self.start.x == self.end.x {
            if self.start.y < self.end.y {
                let mut pts_in_line: Vec<Point> = Vec::new();
                for y_value in self.start.y..=self.end.y {
                    pts_in_line.push(Point {
                        x: self.start.x,
                        y: y_value,
                    })
                }
                return Some(pts_in_line);
            } else {
                let mut pts_in_line: Vec<Point> = Vec::new();
                for y_value in self.end.y..=self.start.y {
                    pts_in_line.push(Point {
                        x: self.start.x,
                        y: y_value,
                    })
                }
                return Some(pts_in_line);
            }
        } else if self.start.y == self.end.y {
            if self.start.x < self.end.x {
                let mut pts_in_line: Vec<Point> = Vec::new();
                for x_value in self.start.x..=self.end.x {
                    pts_in_line.push(Point {
                        x: x_value,
                        y: self.start.y,
                    })
                }
                return Some(pts_in_line);
            } else {
                let mut pts_in_line: Vec<Point> = Vec::new();
                for x_value in self.end.x..=self.start.x {
                    pts_in_line.push(Point {
                        x: x_value,
                        y: self.start.y,
                    })
                }
                return Some(pts_in_line);
            }
        }
        None
    }
}

fn create_line(input: Vec<u16>) -> Line {
    Line {
        start: Point {
            x: input[0],
            y: input[1],
        },
        end: Point {
            x: input[2],
            y: input[3],
        },
    }
}
