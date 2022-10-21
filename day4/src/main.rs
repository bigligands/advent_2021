#![allow(unused_variables)]
#![allow(dead_code)]
use std::fs;

fn main() {
    //GIANT SQUID!!
    // bingo => getting a sequence of numbers and boards
    //need to find which board wins firs
    // boards are 5x5
    let filename = "src/input.txt";
    let raw_input = fs::read_to_string(filename).expect("oops");

    let board_input: Vec<&str> = raw_input.split('\n').collect();

    let mut board_collection: Vec<BingoBoard> = Vec::new();

    let game_number_strings: Vec<&str> = board_input[0].split(',').collect();
    let mut counter = 1;
    let mut board_repository: Vec<&str> = Vec::new();
    for i in 2..board_input.len() {
        if board_input[i].trim().is_empty() {
            continue;
        }
        board_repository.push(board_input[i].trim());
        if counter % 5 == 0 && counter != 0 {
            let range = board_repository.len();
            let board = &board_repository[range - 5..range].to_vec();
            board_collection.push(create_board(board));
        }
        counter += 1;
    }
    //loop, add marks, check for bingo, return board with bingo

    let mut winner = BingoBoard::default();
    let mut final_number: u8 = 0;

    'outer: for number in game_number_strings {
        for board in &mut board_collection[..] {
            let number_value = match u8::from_str_radix(number.trim(), 10) {
                Ok(x) => x,
                Err(_) => panic!("unable to parse {number}"),
            };
            mark_board(&number_value, board); //send an immutable reference of number value
            let bingo = check_for_bingo(&board);
            if bingo {
                winner = board.clone();
                final_number = number_value;
                break 'outer;
            }
        }
    }
    println!("Winner: {:?}", winner);
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
    println!("Final bingo number: {final_number}");
    let final_score = sum
        * match u32::try_from(final_number) {
            Ok(x) => x,
            Err(_) => panic!("unable to parse u8 into u32: {final_number}"),
        };
    println!("Final Score: {final_score}");
}

#[derive(Debug, Default, Clone)]
struct BingoBoard {
    board_matrix: Vec<Vec<BoardPosition>>,
}

// impl Default for BingoBoard {
//     fn default() -> BingoBoard {
//         BingoBoard {
//             board_matrix: Vec::new(),
//         }
//     }
// }

// #[derive(Copy, Clone)]
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
    // println!("marking {number}");
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

    // println!("received raw board: {:?}", raw_board);

    let mut new_board: BingoBoard = BingoBoard {
        board_matrix: Vec::new(),
    };
    for i in 0..5 {
        let mut row: Vec<BoardPosition> = Vec::new();
        let mut split_row = raw_board[i].split(' ').collect::<Vec<&str>>();
        split_row.retain(|x| !x.is_empty());
        for j in 0..5 {
            // println!("attempting u8 from: {}", split_row[j].trim());
            let pos = BoardPosition {
                value: match u8::from_str_radix(split_row[j], 10) {
                    Ok(x) => x,
                    Err(_) => {
                        // println!("Error: {} not parsed", split_row[j].trim());
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
