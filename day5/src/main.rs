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
    // compare collection for overlap -- complete
    // how many points have at least 2 overlaps? -- complete
    // implement diagonal lines
    let split_raw_input = input.split('\n').collect::<Vec<&str>>();
    let mut line_collection: Vec<Line> = Vec::new();
    for line in split_raw_input {
        // 1,2 -> 3,4
        // 1,2,3,4
        let mut trimmed_line = line.replace(&"->", &",").clone();
        trimmed_line.retain(|x| x != ' ');
        let number_strings = trimmed_line.split(',').collect::<Vec<&str>>();
        let mut numbers: Vec<i16> = Vec::new();
        'sloop: for s in number_strings {
            if s.is_empty() {
                continue 'sloop;
            }

            let conversion = match i16::from_str_radix(s.trim(), 10) {
                Ok(x) => x,
                Err(_) => panic!("Failed to convert str to i16: {}", s),
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
    // answer is between 17763 and 18966
    println!("{}", repeated_values.len());
}

#[derive(Eq, Hash, PartialEq, Copy, Clone)]
struct Point {
    x: i16,
    y: i16,
}
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn get_pts_in_line(&self) -> Option<Vec<Point>> {
        let mut pts_in_line: Vec<Point> = Vec::new();
        if self.start.x == self.end.x {
            // vertical line
            if self.start.y < self.end.y {
                for y_value in self.start.y..=self.end.y {
                    pts_in_line.push(Point {
                        x: self.start.x,
                        y: y_value,
                    })
                }
                return Some(pts_in_line);
            } else {
                // reversed
                for y_value in self.end.y..=self.start.y {
                    pts_in_line.push(Point {
                        x: self.start.x,
                        y: y_value,
                    })
                }
                return Some(pts_in_line);
            }
        } else if self.start.y == self.end.y {
            // horizontal line
            if self.start.x < self.end.x {
                for x_value in self.start.x..=self.end.x {
                    pts_in_line.push(Point {
                        x: x_value,
                        y: self.start.y,
                    })
                }
                return Some(pts_in_line);
            } else {
                // reversed
                for x_value in self.end.x..=self.start.x {
                    pts_in_line.push(Point {
                        x: x_value,
                        y: self.start.y,
                    })
                }
                return Some(pts_in_line);
            }
        } else if i16::abs(self.start.x - self.end.x) == i16::abs(self.start.y - self.end.y) {
            //diagonal
            let points = Line::fill_diagonal(self.start, self.end);
            return Some(points);
        }
        None
    }
    fn fill_diagonal(point_one: Point, point_two: Point) -> Vec<Point> {
        let mut point_collection: Vec<Point> = Vec::new();

        // x distance = y distance
        let range = i16::abs(point_one.x - point_two.x);

        // determine direction of x
        let x_comp = point_one.x - point_two.x;
        let x_factor = match x_comp <= 0 {
            true => 1,
            false => -1,
        };
        // determine direction of y
        let y_comp = point_one.y - point_two.y;
        let y_factor = match y_comp <= 0 {
            true => 1,
            false => -1,
        };

        for i in 0..=range {
            let pt = Point {
                x: point_one.x + (i * x_factor),
                y: point_one.y + (i * y_factor),
            };
            point_collection.push(pt);
        }
        println!(
            "1: ({},{}), 2:({},{})",
            point_one.x, point_one.y, point_two.x, point_two.y
        );
        println!(
            "first: ({},{}) last:({},{})",
            point_collection[0].x,
            point_collection[0].y,
            point_collection[point_collection.len() - 1].x,
            point_collection[point_collection.len() - 1].y
        );
        point_collection
    }
}

fn create_line(input: Vec<i16>) -> Line {
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
