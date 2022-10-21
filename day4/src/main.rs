#![allow(unused_variables)]
#![allow(dead_code)]
use std::fs;

fn main() {
    //GIANT SQUID!!
    // bingo => getting a sequence of numbers and boards
    //need to find which board wins first
    // boards are 5x5
    let filename = "src/input.txt";
    let raw_input = match fs::read_to_string(filename) {
        // Match output to enum result
        Ok(s) => s,
        Err(_) => panic!("Unable to parse string from {filename}"), // throw exception
    }; //.expect("oops"); alternative way to parse result
    let board_input: Vec<&str> = raw_input.split('\n').collect(); // collects iterator into a collection
    let mut board_collection: Vec<BingoBoard> = Vec::new();

    let game_number_strings: Vec<&str> = board_input[0].split(',').collect();
    let mut counter = 1; // start at 1 to use %5 on count
    let mut board_repository: Vec<&str> = Vec::new();
    for i in 2..board_input.len() {
        if board_input[i].trim().is_empty() {
            continue;
        }
        board_repository.push(board_input[i].trim());
        if counter % 5 == 0 && counter != 0 {
            // boards are 5 rows
            let range = board_repository.len();
            let board = &board_repository[range - 5..range].to_vec();
        }
        counter += 1;
    }

    let mut winner = BingoBoard::default(); //Bingoboard implements default
    let mut first_winning_number: u8 = 0;

    'outer: for number in &game_number_strings[..] {
        //borrow a slice to avoid moving
        for board in &mut board_collection[..] {
            let number_value = match u8::from_str_radix(number.trim(), 10) {
                // standard numbers are radix base 10, [0..9]
                Ok(x) => x,
                Err(_) => panic!("unable to parse {number}"),
            };
            mark_board(&number_value, board); //send an immutable reference of number value, and a mutable ref of bingoboard
            let bingo = check_for_bingo(&board); //send a reference of board to check for bingo
            if bingo {
                winner = board.clone(); // clone the board and assign to winner var
                first_winning_number = number_value;
                break 'outer; // break labeled loop, default is to break inner loop
            }
        }
    }

    let mut winning_boards: Vec<BingoBoard> = Vec::new();
    let mut winning_numbers: Vec<u8> = Vec::new();

    for number in &game_number_strings[..] {
        for board in &mut board_collection[..] {
            let number_value = match u8::from_str_radix(number.trim(), 10) {
                Ok(x) => x,
                Err(_) => panic!("unable to parse {number}"),
            };
            let mut bingo = check_for_bingo(&board);
            if !bingo {
                mark_board(&number_value, board);
                bingo = check_for_bingo(&board);
                if bingo {
                    // nest this conditional to ensure proper winning number is saved
                    winning_boards.push(board.clone());
                    winning_numbers.push(number_value);
                }
            }
        }
    }

    let last_winner = winning_boards[&winning_boards.len() - 1].clone(); // grab last value from the collection of winning boards
    let last_winning_number = winning_numbers[winning_numbers.len() - 1]; // last value from collection of winning numbers

    let first_winner_score = tally_score(winner, first_winning_number);
    let last_winner_score = tally_score(last_winner, last_winning_number);

    println!("First winning board score: {first_winner_score}");
    println!("Last winning board score: {last_winner_score}");
}

fn tally_score(winner: BingoBoard, final_number: u8) -> u32 {
    let mut sum = 0;
    for row in winner.board_matrix {
        for pos in row {
            if pos.marked == false {
                sum += match u32::try_from(pos.value) {
                    Ok(x) => x,
                    Err(_) => panic!("Couldn't convert u8 to u32: {}", pos.value),
                };
            }
        }
    }
    let final_score = sum
        * match u32::try_from(final_number) {
            Ok(x) => x,
            Err(_) => panic!("unable to parse u8 into u32: {final_number}"),
        };
    final_score
}

#[derive(Debug, Default, Clone)]
struct BingoBoard {
    board_matrix: Vec<Vec<BoardPosition>>,
}

// how to manually implement Default
// impl Default for BingoBoard {
//     fn default() -> BingoBoard {
//         BingoBoard {
//             board_matrix: Vec::new(),
//         }
//     }
// }

#[derive(Debug, Clone)]
struct BoardPosition {
    value: u8,
    marked: bool,
}

fn check_for_bingo(board: &BingoBoard) -> bool {
    // check the whole board to see if any marked properties align 5 in a row
    for i in 0..5 {
        let mut col: Vec<bool> = Vec::new();
        let mut row: Vec<bool> = Vec::new();
        for j in 0..5 {
            col.push(board.board_matrix[i][j].marked);
            row.push(board.board_matrix[j][i].marked);
        }
        if !col.contains(&false) {
            return true;
        }
        if !row.contains(&false) {
            return true;
        }
        col.clear();
        row.clear();
    }
    return false;
}

fn mark_board(number: &u8, board: &mut BingoBoard) {
    for i in 0..5 {
        for j in 0..5 {
            if &board.board_matrix[i][j].value == number {
                board.board_matrix[i][j].marked = true;
            }
        }
    }
}

fn create_board(raw_board: &Vec<&str>) -> BingoBoard {
    //need to split &str into array of strings
    // ["12 13 55 8 12"] -> ["12", "13", "55", "8", "12"]
    let split_raw_board: Vec<Vec<&str>> = Vec::new();

    let mut new_board: BingoBoard = BingoBoard {
        board_matrix: Vec::new(),
    };
    for i in 0..5 {
        let mut row: Vec<BoardPosition> = Vec::new();
        let mut split_row = raw_board[i].split(' ').collect::<Vec<&str>>(); // turbofish operator to indicate type
        split_row.retain(|x| !x.is_empty()); // \x\ is similar to (x) =>
        for j in 0..5 {
            let pos = BoardPosition {
                value: match u8::from_str_radix(split_row[j], 10) {
                    Ok(x) => x,
                    Err(_) => {
                        panic!("couldn't parse {}", split_row[j])
                    }
                },
                marked: false,
            };
            row.push(pos);
        }
        new_board.board_matrix.push(row);
    }
    return new_board;
}
