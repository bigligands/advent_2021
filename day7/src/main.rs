use std::convert::Infallible;
use std::fs;
use std::num;

fn main() {
    let input_path = "src/input.txt";

    let raw_input = match fs::read_to_string(input_path) {
        Ok(s) => s,
        Err(_) => panic!("Unable to parse file {}", input_path),
    };

    let input_str = raw_input.split(',').collect::<Vec<&str>>();
    // let test_input = "16,1,2,0,4,2,7,1,2,14";
    // let input_str = test_input.split(',').collect::<Vec<&str>>();

    let mut input: Vec<u16> = Vec::new();

    let len = input_str.len();
    for i in 0..len {
        input.push(match u16::from_str_radix(input_str[i].trim(), 10) {
            Ok(u) => u,
            Err(_) => panic!("oh god"),
        })
    }

    let mut heights: Vec<u32> = Vec::new();
    let max = input.iter().max().unwrap();
    for i in 0..*max {
        heights.push(sum_fuel(&input, i as u16));
    }
    println!(
        "Most efficient height fuel cost: {}",
        heights.iter().min().unwrap()
    );
}

// Sum fuel recursive
fn fuel_cost(x: u32) -> u32 {
    if x == 0 || x == 1 {
        return x;
    } else {
        return x + fuel_cost(x - 1);
    }
}

fn sum_fuel(y_collection: &Vec<u16>, height: u16) -> u32 {
    let mut fuel: u32 = 0;
    for i in 0..y_collection.len() {
        fuel += match height >= y_collection[i] {
            true => fuel_cost((height - y_collection[i]) as u32),
            false => fuel_cost((y_collection[i] - height) as u32),
        };
    }
    fuel
}
