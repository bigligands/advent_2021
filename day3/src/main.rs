use std::fs;

fn main() {
    let filename = "src/input.txt";
    let raw_input = fs::read_to_string(filename).expect("oops");

    let string_byte_vector: Vec<&str> = raw_input.split('\n').collect();
    let mut trimmed_byte_vector: Vec<Vec<&str>> = Vec::new();

    let mut count_vector: Vec<BitCount> = Vec::new();

    for line in &string_byte_vector[..] {
        let l: Vec<&str> = line.split("").filter(|c| c.contains(['1', '0'])).collect();
        // println!("{:?}", l);
        if l.len() > 1 {
            trimmed_byte_vector.push(l);
        }
    }

    let mut gamma_rate: Vec<&str> = Vec::new();
    let mut epsilon_rate: Vec<&str> = Vec::new();

    for i in 0..12 {
        let mut counter = BitCount { zero: 0, one: 0 };
        for line in &trimmed_byte_vector[..] {
            if line[i] == "0" {
                counter.one += 1;
            } else {
                counter.zero += 1;
            }
        }
        let new_counter = BitCount { ..counter };
        count_vector.push(new_counter);
    }

    for vect in count_vector {
        if vect.zero > vect.one {
            gamma_rate.push("0");
            epsilon_rate.push("1");
        } else {
            gamma_rate.push("1");
            epsilon_rate.push("0");
        }
    }

    let gamma = gamma_rate.concat();
    let epsilon = epsilon_rate.concat();

    let gamma_value = u32::from_str_radix(&gamma, 2).unwrap();
    let epsilon_value = u32::from_str_radix(&epsilon, 2).unwrap();
    println!("Power rate = {}", &gamma_value * &epsilon_value);
}

struct BitCount {
    zero: u32,
    one: u32,
}
