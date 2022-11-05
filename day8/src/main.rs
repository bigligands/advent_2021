use std::fs;

fn main() {
    let path = "input.txt";
    let input = fs::read_to_string(path).unwrap();
    let displays = day8::parse_input(&input[..]);
    let mut sum = 0;

    for display in displays {
        sum += display.decipher_all_inputs();
    }

    println!("Sum of unique outputs: {sum}");
}
