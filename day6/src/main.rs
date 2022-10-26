#![allow(unused_variables)]
use std::{collections::HashMap, fs};

fn main() {
    const PATH: &str = "src/input.txt";
    let raw_input = match fs::read_to_string(PATH) {
        Ok(s) => s,
        Err(_) => panic!("Could not parse file {} into string.", PATH),
    };
    let mut school_of_fish = parse_fish_nums(&raw_input);

    for i in 0..256 {
        adjust_day(&mut school_of_fish);
    }

    let mut fish_counter = 0;
    for i in 0..=8u8 {
        let count = school_of_fish.get(&i).unwrap();
        fish_counter += *count;
    }
    println!("Total fish: {}", fish_counter);
}

fn parse_fish_nums(input: &str) -> HashMap<u8, u64> {
    let mut fish_catalog: HashMap<u8, u64> = HashMap::new();
    for i in 0..8u8 {
        fish_catalog.insert(i, 0);
    }
    // dictionary of {0: n, 1: n, 2: n} etc..

    //# NOTE:
    // looked up a hint for part 2 and saw it was possible to use an array with the len of possible timers
    //      as a way to store fish counts, using the position as an indicator of the timer.
    // incredibly clever concept to use the minimum possible representation
    // allows for much faster execution and memory efficiency

    let split_input = input.split(',').collect::<Vec<&str>>();
    for raw_timer in split_input {
        let fish_timer = match u8::from_str_radix(raw_timer.trim(), 10) {
            Ok(f) => f,
            Err(_) => {
                println!("failed to parse {raw_timer}");
                continue;
            }
        };
        fish_catalog
            .entry(fish_timer)
            .and_modify(|n| *n += 1)
            .or_insert(0);
    }
    fish_catalog
}

fn adjust_day(catalog: &mut HashMap<u8, u64>) {
    let resetting = match catalog.get(&0u8) {
        Some(v) => *v,
        None => panic!("ahhh"),
    };

    for i in 1..=8u8 {
        let transfer = match catalog.get(&i) {
            Some(v) => *v,
            None => 0,
        };
        catalog.insert(
            i - 1u8,
            match catalog.get(&i) {
                Some(x) => *x,
                None => 0,
            },
        );
    }
    // 0's start back at 6, add to count from 7 descending
    catalog.entry(6u8).and_modify(|x| *x += resetting);
    // new fish start at 8, no other fish will be there
    catalog.insert(8u8, resetting);
}
