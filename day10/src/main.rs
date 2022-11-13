use std::fs;

fn main() {
    // let input_path = "test_input.txt";
    let input_path = "input.txt";
    let input = fs::read_to_string(input_path).unwrap();

    let mut errors = Vec::new();

    for line in input.lines() {
        let mut last_open = Vec::new();
        let chars = line.chars().collect::<Vec<char>>();
        for symbol in chars {
            if symbol == '[' {
                last_open.push('[')
            } else if symbol == ']' {
                match last_open.pop() {
                    Some(c) => {
                        if c != '[' {
                            errors.push(57);
                        }
                    }
                    None => errors.push(57),
                }
            } else if symbol == '(' {
                last_open.push('(');
            } else if symbol == ')' {
                match last_open.pop() {
                    Some(c) => {
                        if c != '(' {
                            errors.push(3);
                        }
                    }
                    None => errors.push(3),
                }
            } else if symbol == '{' {
                last_open.push('{');
            } else if symbol == '}' {
                match last_open.pop() {
                    Some(c) => {
                        if c != '{' {
                            errors.push(1197);
                        }
                    }
                    None => errors.push(1197),
                }
            } else if symbol == '<' {
                last_open.push('<');
            } else if symbol == '>' {
                match last_open.pop() {
                    Some(c) => {
                        if c != '<' {
                            errors.push(25137);
                        }
                    }
                    None => errors.push(25137),
                }
            }
        }
    }
    // println!("Errors: {:?}", errors);
    println!("{}", errors.iter().sum::<u32>());
}
