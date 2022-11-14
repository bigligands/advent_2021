use std::fs;

fn main() {
    // let input_path = "test_input.txt";
    let input_path = "input.txt";
    let input = fs::read_to_string(input_path).unwrap();

    let incomplete_lines = input
        .lines()
        .map(|line| {
            line.chars().scan(Vec::new(), |lastopen, c| {
                Some(match c {
                    '(' | '[' | '{' | '<' => {
                        lastopen.push(c);
                        Some(c)
                    }
                    ')' => match lastopen.pop().unwrap() {
                        '(' => Some(c),
                        _ => None, // push None if corrupted
                    },
                    ']' => match lastopen.pop().unwrap() {
                        '[' => Some(c),
                        _ => None,
                    },
                    '}' => match lastopen.pop().unwrap() {
                        '{' => Some(c),
                        _ => None,
                    },
                    '>' => match lastopen.pop().unwrap() {
                        '<' => Some(c),
                        _ => None,
                    },
                    _ => None,
                })
            })
        })
        // remove any iterators that have None as they are corrupted
        .filter(|c| !c.to_owned().any(|b| b.is_none()))
        // collect the char values from the Option<char> which should all be Some due to filter
        .map(|c| c.map(|a| a.unwrap()).collect::<Vec<char>>())
        // collect each vector within a vector
        .collect::<Vec<Vec<char>>>();

    // want to iterate over incomplete line, then remove any pairs already present
    let mut still_needs_to_close = incomplete_lines.to_owned();
    for i in 0..incomplete_lines.len() {
        for c in &incomplete_lines[i] {
            match c {
                ')' | ']' | '}' | '>' => {
                    if let Some(pos) = still_needs_to_close[i].iter().position(|x| *x == *c) {
                        // remove the closing and the opening
                        still_needs_to_close[i].remove(pos);
                        still_needs_to_close[i].remove(pos - 1);
                    };
                }
                _ => continue,
            }
        }
    }

    let closing_symbols = still_needs_to_close
        .iter()
        .map(|x| {
            x.iter()
                .map(|c| match c {
                    '(' => ')',
                    '[' => ']',
                    '{' => '}',
                    '<' => '>',
                    _ => panic!("illegal character detected."),
                })
                .rev()
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut scores = closing_symbols
        .iter()
        .map(|x| {
            x.iter().fold(0, |score: u64, x| {
                5 * score
                    + match x {
                        ')' => 1,
                        ']' => 2,
                        '}' => 3,
                        '>' => 4,
                        _ => 0,
                    }
            })
        })
        .collect::<Vec<_>>();
    scores.sort();

    // all of the collecting and reiterating can likely be combined

    // since index is 0 based, don't need to add 1 to get median value
    println!("{}", scores[scores.len() / 2]);
}
