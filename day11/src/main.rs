#![allow(dead_code)]
use std::{collections::HashSet, fs};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut grid = OctopusGrid::new(input);

    let mut scanning = true;
    let mut count = 0;
    while scanning {
        grid.step();
        count += 1; // track the steps
        grid.scan_for_reactions();
        grid.coordinate_stack.clear();

        grid.grid.iter().for_each(|x| println!("{:?}", x));
        println!();

        if grid.flashes_this_cycle == 100 {
            println!("all flashed: {}", count);
            grid.flashes_this_cycle = 0;
            scanning = false;
        } else {
            grid.flashes_this_cycle = 0;
        }
    }
    println!("flashes: {}", grid.total_flashes);
}

struct OctopusGrid {
    grid: Vec<Vec<u8>>,
    coordinate_stack: HashSet<(usize, usize)>,
    total_flashes: u32,  // count flashes
    reaction_queue: u32, // keep track of reactions
    flashes_this_cycle: u32,
}

impl OctopusGrid {
    fn new(input: String) -> OctopusGrid {
        return OctopusGrid {
            grid: Vec::from_iter(input.lines().scan(0u8, |i, x| {
                // Create hashmap from lines
                let grid = x
                    .chars()
                    .map(|c| c.to_digit(10)) // convert char to int
                    .filter(|o| o.is_some()) // filter out non-digits
                    .map(|i| i.unwrap() as u8) // grap value from Option
                    .collect::<Vec<u8>>();
                *i += 1; // increase state counter
                Some(grid) // return HashMap as Option
            })),
            coordinate_stack: HashSet::new(),
            total_flashes: 0,
            reaction_queue: 0,
            flashes_this_cycle: 0,
        };
    }

    // step up each value in the grid and collect the energy releases
    fn step(&mut self) {
        let mut stepped_grid = Vec::new();
        let coordinates = self // completely using iterators, no loops
            .grid
            .iter()
            .enumerate()
            .fold(Vec::new(), |mut coords, (i, x)| {
                stepped_grid.push(x.iter().map(|x| (x + 1) % 10).collect::<Vec<_>>());
                x.iter()
                    .enumerate()
                    .map(|(j, value)| (j, (value + 1) % 10))
                    .for_each(|(k, value)| {
                        if value == 0 {
                            coords.push((i, k));
                            self.total_flashes += 1; // tally of flashes
                            self.reaction_queue += 1; // tally of scan cycles, will be reset after each cycle
                            self.flashes_this_cycle += 1; // count flashes initial step
                        }
                    });
                coords // return the coordinates
            });
        self.grid = stepped_grid; // replaces grid with grid + 1
        coordinates.iter().for_each(|x| {
            self.coordinate_stack.insert(*x); // add coordinates to the stack
        });
    }

    fn scan_for_reactions(&mut self) {
        while self.reaction_queue > 0 {
            // get positions adjacent to coordinate stack
            let mut adjacent_positions = Vec::new();
            for coordinate in self.coordinate_stack.iter() {
                let adj = OctopusGrid::grab_adjacent_positions(coordinate);
                adjacent_positions.push(adj);
            }

            let adjacent_positions = adjacent_positions // flatten nested positions
                .iter()
                .flatten()
                .map(|x| *x)
                .collect::<Vec<(usize, usize)>>();

            self.reaction_queue = 0; // reset queue
            self.coordinate_stack.clear(); // remove collection of flashed positions

            // step diag by 1, retaining 10s
            adjacent_positions
                .iter()
                .for_each(|(row, col)| match (self.grid[*row][*col]) + 1 {
                    1 => self.grid[*row][*col] = 0, // stepping newly flashed positions
                    x => {
                        if x < 11 {
                            self.grid[*row][*col] = x; // step with no flash
                        } else {
                            self.grid[*row][*col] = 10; // rollback anything higher than 10
                        }
                    }
                });

            (0..=9).into_iter().for_each(|row_index| {
                (0..=9).into_iter().for_each(|pos_index| {
                    if self.grid[row_index][pos_index] == 10 {
                        self.reaction_queue += 1; // continue loop
                        self.total_flashes += 1; // count flashes
                        self.coordinate_stack.insert((row_index, pos_index)); // store flash
                                                                              // locations
                        self.flashes_this_cycle += 1;
                        self.grid[row_index][pos_index] = 0; // set 10s to 0s
                    }
                })
            })
        }
    }

    fn grab_adjacent_positions(pos: &(usize, usize)) -> Vec<(usize, usize)> {
        let mut adjacent_positions: Vec<(usize, usize)> = Vec::new();
        let start = pos.0 as i32;
        let end = pos.1 as i32;
        (start - 1..=start + 1).for_each(|i| {
            (end - 1..=end + 1).for_each(|j| {
                if -1 < i
                    && i < 10
                    && -1 < j
                    && j < 10
                    && ((i as usize, j as usize) != (pos.0, pos.1))
                {
                    adjacent_positions.push((i as usize, j as usize));
                }
            })
        });
        adjacent_positions
    }
}
