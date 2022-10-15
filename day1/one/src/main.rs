use std::env;
use std::fs;

fn main() {
    let filename = "src/depths.txt";
    let depths = fs::read_to_string(filename).expect("didn't work");
    let split: Vec<&str> = depths.split('\n').collect();
    let mut split_nums: Vec<u32> = Vec::new();
    for depth in &split {
        split_nums.push(match depth.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        });
    }
    //part 1
    // let mut prev: u32 = 0;
    // let mut count = 0;
    // let mut first = true;
    // let a = &split;
    // for depth in a {
    //     let depth_num: u32 = match depth.trim().parse() {
    //         Ok(num) => num,
    //         Err(_) => continue,
    //     };

    //     if first == true {
    //         first = false;
    //         prev = depth_num;
    //         continue;
    //     }

    //     if depth_num > prev {
    //         println!("{depth_num} is deeper than {prev}");
    //         count += 1;
    //     }

    //     prev = depth_num;
    // }

    // println!("{count}");
    let total: usize = split.len();
    // println!("{total}");

    let mut index = 1;
    let mut prev = sliding_depth(&split_nums[..3]);
    let mut count = 0;
    while index + 3 < total {
        let point = sliding_depth(&split_nums[index..=index + 2]);
        if point > prev {
            count += 1;
        }
        prev = point;
        index += 1;
    }
    println!("Sliding depth method count: {count}");
}

fn sliding_depth(data: &[u32]) -> u32 {
    let mut sum = 0;
    for point in data {
        sum += point;
    }
    sum
}
