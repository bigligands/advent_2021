use std::collections::{HashMap, HashSet};

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
    // part 1
    pub fn sum_unique_outputs(&self) -> u32 {
        // [1,4,7,8] have lengths [2,4,3,7]
        // remaining : 0,2,3,5,6,9
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

    // part 2
    fn decipher_unique_inputs(&self) -> HashMap<u8, HashSet<u8>> {
        let mut map: HashMap<u8, HashSet<u8>> = HashMap::new();
        let mut cache: [u8; 4] = [0; 4];
        for segment in [&self.input[..], &self.output[..]].concat() {
            let length = segment.len();
            if length == 2 && cache[0] == 0 {
                let charbytes = segment.as_bytes().iter().copied().collect::<HashSet<u8>>();
                map.insert(1, charbytes);
                cache[0] = 1;
            } else if length == 3 && cache[1] == 0 {
                let chars = segment.as_bytes().iter().copied().collect::<HashSet<u8>>();
                map.insert(7, chars);
                cache[1] = 1;
            } else if length == 4 && cache[2] == 0 {
                let chars = segment.as_bytes().iter().copied().collect::<HashSet<u8>>();
                map.insert(4, chars);
                cache[2] = 1;
            } else if length == 7 && cache[3] == 0 {
                let chars = segment.as_bytes().iter().copied().collect::<HashSet<u8>>();
                map.insert(8, chars);
                cache[3] = 1;
            }
            if cache == [1; 4] {
                break; // stop loop when all unique numbers determined
            }
        }
        map
    }

    // part 2
    pub fn decipher_all_inputs(&self) -> u32 {
        // get unique length characters mapped
        let mut char_map = self.decipher_unique_inputs();
        // track whether all 10 numbers have been deciphered
        let mut cache: [u8; 10] = [0; 10];
        // mark the unique numbers as deciphered
        for pos in [1, 4, 7, 8] {
            cache[pos] = 1;
        }

        // loop over combined input and output
        for segment in [&self.input[..], &self.output[..]].concat() {
            // non-unique numbers are either lenght 5 or 6
            if segment.len() != 6 && segment.len() != 5 {
                continue;
            }
            // if the cache is full break the loop
            // could put this at the end for minor optimization
            if cache.eq(&[1u8; 10]) {
                break; // all characters mapped
            }

            // convert the string representation of number into hashset of char bytes
            let set: HashSet<u8> = segment.as_bytes().iter().copied().collect::<HashSet<u8>>();

            // If Else block for the logic to deduce the remaining numbers
            if Self::get_difference(&set, &char_map.get(&4).unwrap(), 0, 6) {
                let nine = set.iter().copied().collect::<HashSet<u8>>();
                char_map.insert(9, nine);
                cache[9] = 1;
            } else if char_map
                .get(&8)
                .unwrap()
                .difference(&set)
                .collect::<HashSet<&u8>>()
                == char_map
                    .get(&4)
                    .unwrap()
                    .difference(&set)
                    .collect::<HashSet<&u8>>()
                && char_map.get(&7).unwrap().difference(&set).count() == 0
                && set.len() == 6
            {
                // 8.diff(x) == 4.diff(x) | 7.diff(x) = 0 positions | x.len = 6
                let zero = set.iter().copied().collect::<HashSet<u8>>();
                char_map.insert(0, zero);
                cache[0] = 1;
            } else if (char_map.get(&1).unwrap()).difference(&set).count() == 0
                && Self::get_difference(&set, &char_map.get(&4).unwrap(), 1, 5)
            {
                // 1.diff(x) = nothing | 4.diff(x) = 1 position | x.len = 5
                let three = set.iter().copied().collect::<HashSet<u8>>();
                char_map.insert(3, three);
                cache[3] = 1;
            } else if Self::get_difference(&set, &char_map.get(&4).unwrap(), 2, 5) {
                // 4.diff(x) = 2 positions | x.len = 5
                let two = set.iter().copied().collect::<HashSet<u8>>();
                char_map.insert(2, two);
                cache[2] = 1;
            } else if Self::get_difference(&set, &char_map.get(&4).unwrap(), 1, 5) {
                // 4.diff(x) = 1 position | x.len = 5
                let five = set.iter().copied().collect::<HashSet<u8>>();
                char_map.insert(5, five);
                cache[5] = 1;
            } else if Self::get_difference(&set, &char_map.get(&7).unwrap(), 1, 6) {
                // 7.diff(x) = 1 position | x.len = 6
                let six = set.iter().copied().collect::<HashSet<u8>>();
                char_map.insert(6, six);
                cache[6] = 1;
            }
        }

        let mut out = [0u8; 4]; // 4 digit output
        let mut i = 0usize; // loop counter
                            // need to get key from value to convert string bytes to number
        for num in &self.output[..] {
            out[i] = Self::find_key_from_val(
                &char_map, // immutable ref to the char map
                num.as_bytes().into_iter().copied().collect::<HashSet<u8>>(),
                // output string, to bytes
            )
            .unwrap();
            i += 1;
        }
        println!("{:?}", out); // display the array representation of output value
        let return_value = (out[0] as u32 * 1000) // convert array to u32
            + (out[1] as u32 * 100)
            + (out[2] as u32 * 10)
            + (out[3] as u32 * 1);
        println!("{return_value}");
        return_value
    }

    fn find_key_from_val(map: &HashMap<u8, HashSet<u8>>, value: HashSet<u8>) -> Option<u8> {
        let res = map
            .iter()
            .find_map(|(k, v)| if value == *v { Some(*k) } else { None });
        res
    }

    fn get_difference(
        set: &HashSet<u8>,
        other: &HashSet<u8>,
        diff_len: usize,
        set_len: usize,
    ) -> bool {
        other.difference(&set).count() == diff_len && set.len() == set_len
    }

    // plan !!
    // 1, 4, 7, 8 are done
    // only ones left that matter are of len 5 and 6
    // just find rules to distinguish them
    // 3 shares 1 and diff 4 is 1
    // 9 has no diff with 4
    // 0 diff 8 has same diff as 0 diff 4, plus no diff with 7
    // 2 diff 4 has 2
    // 5 diff 4 has 1
    // 6 diff 7 is 1
}

// Testing
//--------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

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

        assert_eq!(test_result, test_input.sum_unique_outputs());
    }

    #[test]
    fn test_advent_sample() {
        let path = "test_input.txt";
        let input = fs::read_to_string(path).unwrap();
        let displays = parse_input(&input[..]);
        let mut sum = 0;
        for display in displays {
            sum += display.sum_unique_outputs();
        }
        println!("{}", sum);
        assert_eq!(sum, 26);
    }

    #[test]
    fn zero_condition() {
        let zero = HashSet::from([b'a', b'b', b'c', b'e', b'f', b'g']);
        let four = HashSet::from([b'b', b'c', b'd', b'f']);
        let eight = HashSet::from([b'a', b'b', b'c', b'd', b'e', b'f', b'g']);
        let seven = HashSet::from([b'a', b'c', b'f']);

        assert_eq!(
            eight.difference(&zero).collect::<HashSet<_>>(),
            four.difference(&zero).collect::<HashSet<_>>()
        );

        assert_eq!(seven.difference(&zero).count(), 0);
        assert_eq!(Display::get_difference(&zero, &seven, 0, 6), true);
    }

    #[test]
    fn three_condition() {
        let one = HashSet::from([b'c', b'f']);
        let three = HashSet::from([b'a', b'c', b'd', b'f', b'g']);
        let four = HashSet::from([b'b', b'c', b'd', b'f']);

        assert_eq!(one.difference(&three).count(), 0);
        assert_eq!(four.difference(&three).count(), 1);
        assert_eq!(Display::get_difference(&three, &one, 0, 5), true);
        assert_eq!(Display::get_difference(&three, &four, 1, 5), true);
    }
}
