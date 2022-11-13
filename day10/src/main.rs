use std::fs;

fn main() {
    // let input_path = "test_input.txt";
    let input_path = "input.txt";
    let input = fs::read_to_string(input_path).unwrap();

    let a = input
        .lines()
        .map(|line| {
            line.chars()
                .scan(Vec::new(), |lastopen, c| match c {
                    '(' | '[' | '{' | '<' => {
                        lastopen.push(c);
                        Some(0)
                    }
                    ')' => match lastopen.pop().unwrap() {
                        '(' => Some(0),
                        _ => Some(3),
                    },
                    ']' => match lastopen.pop().unwrap() {
                        '[' => Some(0),
                        _ => Some(57),
                    },
                    '}' => match lastopen.pop().unwrap() {
                        '{' => Some(0),
                        _ => Some(1197),
                    },
                    '>' => match lastopen.pop().unwrap() {
                        '<' => Some(0),
                        _ => Some(25137),
                    },
                    _ => Some(0),
                })
                .sum::<u32>()
        })
        .sum::<u32>();

    println!("{}", a);
}
