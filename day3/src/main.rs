use std::fs;

fn main() {
    let filename = "src/input.txt";
    let raw_input = fs::read_to_string(filename).expect("oops");

    let string_byte_vector: Vec<&str> = raw_input.split('\n').collect();
    let mut trimmed_byte_vector: Vec<Vec<&str>> = Vec::new();

    let mut count_vector: Vec<BitCount> = Vec::new();

    for line in &string_byte_vector[..] {
        let l: Vec<&str> = line.split("").filter(|c| c.contains(['1', '0'])).collect();
        if l.len() > 1 {
            trimmed_byte_vector.push(l);
        }
    }

    let mut gamma_rate: Vec<&str> = Vec::new();
    let mut epsilon_rate: Vec<&str> = Vec::new();

    for i in 0..12 {
        let mut counter = BitCount { zero: 0, one: 0 };
        for line in &trimmed_byte_vector[..] {
            if line[i] == "1" {
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

    //part 2
    let mut oxygen_rating_vec: Vec<Vec<&str>> = Vec::new();
    let mut co2_rating_vec: Vec<Vec<&str>> = Vec::new();
    trimmed_byte_vector.clone_into(&mut oxygen_rating_vec);
    trimmed_byte_vector.clone_into(&mut co2_rating_vec);

    let mut i = 0;
    while oxygen_rating_vec.len() > 1 {
        let mut counter = BitCount { zero: 0, one: 0 };
        for line in &oxygen_rating_vec[..] {
            if line[i] == "1" {
                counter.one += 1;
            } else {
                counter.zero += 1;
            }
        }

        if counter.one > counter.zero {
            if oxygen_rating_vec.len() > 1 {
                oxygen_rating_vec.retain(|c| c[i] == "1");
            }
        }
        if counter.one < counter.zero {
            if oxygen_rating_vec.len() > 1 {
                oxygen_rating_vec.retain(|c| c[i] == "0");
            }
        } else {
            if oxygen_rating_vec.len() > 1 {
                oxygen_rating_vec.retain(|c| c[i] == "1");
            }
        }
        i += 1;
    }

    let mut i = 0;
    while co2_rating_vec.len() > 1 {
        let mut counter = BitCount { zero: 0, one: 0 };
        for line in &co2_rating_vec[..] {
            if line[i] == "1" {
                counter.one += 1;
            } else {
                counter.zero += 1;
            }
        }

        if counter.one > counter.zero {
            if co2_rating_vec.len() > 1 {
                co2_rating_vec.retain(|c| c[i] == "0");
            }
        }
        if counter.one < counter.zero {
            if co2_rating_vec.len() > 1 {
                co2_rating_vec.retain(|c| c[i] == "1");
            }
        } else {
            if co2_rating_vec.len() > 1 {
                co2_rating_vec.retain(|c| c[i] == "0");
            }
        }
        i += 1;
    }

    let oxygen_generator_rating = oxygen_rating_vec[0].concat();
    let co2_scrubber_rating = co2_rating_vec[0].concat();
    let oxygen_generator_rating_value = u32::from_str_radix(&oxygen_generator_rating, 2).unwrap();

    let co2_scrubber_rating_value = u32::from_str_radix(&co2_scrubber_rating, 2).unwrap();
    println!(
        "Life support rating: {}",
        oxygen_generator_rating_value * co2_scrubber_rating_value
    );
}

#[derive(Debug)]
struct BitCount {
    zero: u32,
    one: u32,
}
