#![allow(unused_variables)]
use std::fs;

fn main() {
    const PATH: &str = "src/input.txt";
    // let test_input = "3,4,3,1,2";
    let raw_input = match fs::read_to_string(PATH) {
        Ok(s) => s,
        Err(_) => panic!("Could not parse file {} into string.", PATH),
    };
    let mut school_of_fish = parse_fish(&raw_input);
    // let mut school_of_fish = parse_fish(&test_input);
    println!("Initially there are {} fish", school_of_fish.len());

    for i in 0..80 {
        let mut new_fish_collection: Vec<LanternFish> = Vec::new();
        for fish in &mut school_of_fish[..] {
            let new_fish = fish.adjust_timer();
            if let Some(baby_fish) = new_fish {
                new_fish_collection.push(baby_fish);
            }
        }
        school_of_fish.extend(new_fish_collection.into_iter());
    }

    // for i in 0..18 {
    //     let mut new_fish_collection: Vec<LanternFish> = Vec::new();
    //     for fish in &mut school_of_fish[..] {
    //         let new_fish = fish.adjust_timer();
    //         if let Some(baby_fish) = new_fish {
    //             new_fish_collection.push(baby_fish);
    //         }
    //     }
    //     school_of_fish.extend(new_fish_collection.into_iter());
    // }
    println!("After 80 days, there are now {} fish", school_of_fish.len());
}

fn parse_fish(input: &str) -> Vec<LanternFish> {
    let split_input = input.split(',').collect::<Vec<&str>>();
    let mut fish_collection: Vec<LanternFish> = Vec::new();
    for raw_timer in split_input {
        let fish_timer = match u8::from_str_radix(raw_timer.trim(), 10) {
            Ok(f) => f,
            Err(_) => {
                println!("failed to parse {raw_timer}");
                continue;
            }
        };
        fish_collection.push(LanternFish { timer: fish_timer });
    }
    fish_collection
}

#[derive(Debug)]
struct LanternFish {
    timer: u8,
}

impl LanternFish {
    fn adjust_timer(&mut self) -> Option<LanternFish> {
        if self.timer > 0 {
            self.timer -= 1;
            None
        } else {
            self.timer = 6;
            Some(LanternFish { timer: 8 })
        }
    }
}
