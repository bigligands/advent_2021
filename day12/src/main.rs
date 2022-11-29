#![allow(dead_code)]
#![allow(unused_variables, unused_mut)]

use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    // let input = fs::read_to_string("test_input_1.txt").unwrap();
    let input = fs::read_to_string("test_input_2.txt").unwrap();
    // let input = fs::read_to_string("input.txt").unwrap();

    let connections = input.lines().fold(HashMap::new(), |mut conn, line| {
        // connections has each cave as key and connections as Set
        let caves = line.split('-').collect::<Vec<_>>(); // values of the 2 caves
        line.split('-').for_each(|x| {
            if !conn.contains_key(&x.to_string()) {
                conn.insert(x.to_string(), HashSet::<String>::new());
            }
        });
        conn.get_mut(caves[0]).unwrap().insert(caves[1].to_string()); // add the value to the set
                                                                      // for each key
        conn.get_mut(caves[1]).unwrap().insert(caves[0].to_string());
        conn // return the HashMap back to the fold iterator
    });
    println!("{:?}", connections);

    let mut seed = String::from("start");
    let mut paths = Vec::<String>::new();
    let mut completed_paths = Vec::new();
    let mut tails = Vec::new();
    let mut count = 0;

    loop {
        let mut t: String;
        let mut recent_tail: Vec<String>;
        let mut new_paths: Vec<String> = Vec::new();
        if tails.len() != 0 {
            recent_tail = tails.last().cloned().unwrap();
            recent_tail.sort();
            recent_tail.dedup();

            for tail in recent_tail {
                match tail.as_str() {
                    "end" => continue,
                    _ => {}
                }

                let mut cached_connections = Vec::new();
                for con in connections.get(&tail).unwrap() {
                    if cached_connections.contains(&con.as_bytes()) {
                        continue;
                    }

                    match con.as_str() {
                        "start" => continue,
                        x => {}
                    }
                    paths.sort();
                    paths.dedup();

                    // get paths that end with tail
                    // push connections
                    let mut relevant_paths = paths
                        .iter()
                        .filter(|x| x.split(",").last().unwrap().as_bytes() == tail.as_bytes())
                        .filter(|x| x.split(",").last().unwrap().as_bytes() != b"end")
                        .collect::<Vec<_>>();

                    if relevant_paths.len() == 0 {
                        continue;
                    }

                    let filtered_rps = filter_caves(relevant_paths);

                    if filtered_rps.len() == 0 {
                        continue;
                    }
                    for rp in filtered_rps {
                        let mut relevant_path = rp.clone();
                        relevant_path.push_str(",");
                        relevant_path.push_str(con.as_str());
                        new_paths.push(relevant_path.to_string());
                    }
                    cached_connections.push(con.as_bytes());
                }
            }
            // copy all of the paths with the new connections, wipe the old paths
            paths = new_paths.clone();
            for p in new_paths {
                if p.split(",").last().unwrap().as_bytes() == b"end" {
                    completed_paths.push(p);
                }
            }
        } else {
            // first loop
            for con in connections.get(&seed).unwrap() {
                t = seed.clone();
                t.push_str(",");
                t.push_str(con);
                paths.push(t);
            }
        }

        let mut tail = paths // last cave visited
            .iter()
            .map(|x| x.split(","))
            .map(|x| x.map(|x| x.to_string()).last().unwrap())
            .collect::<Vec<_>>();

        if tail.iter().all(|x| x.as_bytes() == b"end") {
            // if every cave visited was the ending, break the loop
            break;
        }
        tails.push(tail);
        count += 1;
        println!("loop {}", count);
    }
    // println!("completed paths: {:?}", completed_paths);
    println!("{} paths found", completed_paths.len());
}

fn filter_caves(caves: Vec<&String>) -> Vec<String> {
    let mut filtered_caves = Vec::new();
    for cave in caves {
        let mut small_caves = cave
            .split(",")
            .filter(|c| c.to_lowercase().as_bytes() == c.as_bytes())
            .map(|c| c.to_string())
            .collect::<Vec<_>>();
        let before_count = small_caves.len();
        small_caves.sort();
        small_caves.dedup();
        let after_count = small_caves.len();
        if before_count - after_count > 1 {
            continue;
        } else {
            filtered_caves.push(cave.to_string());
        }
    }

    filtered_caves
}
