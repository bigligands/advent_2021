#![allow(dead_code)]
#![allow(unused_variables)]

pub fn parse_input(input: &str) -> Vec<Display<Vec<&str>>> {
    let mut display_collection: Vec<Display<Vec<&str>>> = Vec::new();
    for line in input.lines() {
        // split the input into lines
        let input_line = line.split("|").collect::<Vec<&str>>();
        // capture left half as input
        let inputs: Vec<&str> = input_line[0].split_whitespace().map(|x| x.trim()).collect();
        // capture right half as output
        let outputs: Vec<&str> = input_line[1].split_whitespace().map(|x| x.trim()).collect();
        // add to collection
        display_collection.push(Display {
            input: inputs,
            output: outputs,
        });
    }
    display_collection
}

#[derive(Debug)]
pub struct Display<T> {
    pub input: T,
    pub output: T,
}

impl Display<Vec<&str>> {
    pub fn decipher_unique_outputs(&self) -> u32 {
        // [1,4,7,8] have lengths [2,4,3,7]
        let mut outputs: [u32; 4] = [0; 4];
        for segment in &self.output {
            let length = segment.len() as u32;
            println!("{length}");
            if length == 2 {
                outputs[0] += 1;
            } else if length == 4 {
                outputs[1] += 1;
            } else if length == 3 {
                outputs[2] += 1;
            } else if length == 7 {
                outputs[3] += 1;
            }
        }
        let mut sum = 0;
        for x in outputs {
            sum += x;
        }
        sum
    }
}

//--------------------------

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn create_display() {
        let test_input = "abc def | abc";
        let test_display = Display {
            input: vec!["abc", "def"],
            output: vec!["abc"],
        };
        let fn_test = parse_input(test_input);
        dbg!(fn_test);
        // check input
        for i in 0..test_display.input.len() {
            assert_eq!(test_display.input[i], parse_input(test_input)[0].input[i]);
        }
        // check output
        assert_eq!(test_display.output[0], parse_input(test_input)[0].output[0]);
    }

    #[test]
    fn test_decipher_unique_outputs() {
        let test_input = Display {
            input: vec!["abc"],
            output: vec!["fdgacbe", "abc", "gd", "gebd", "a", "a", "a", "a"],
        };
        let test_result: u32 = 4;

        assert_eq!(test_result, test_input.decipher_unique_outputs());
    }

    #[test]
    fn test_advent_sample() {
        let path = "test_input.txt";
        let input = fs::read_to_string(path).unwrap();
        let displays = parse_input(&input[..]);
        let mut sum = 0;
        for display in displays {
            sum += display.decipher_unique_outputs();
        }
        println!("{}", sum);
        assert_eq!(sum, 26);
    }
}
