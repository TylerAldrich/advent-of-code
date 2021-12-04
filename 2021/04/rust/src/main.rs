use advent_input_parser::parse_by_line;
use std::io;

// The bingo board is a 5x5 array of BingoSquares
// Represented as a Vec because "[[BingoSquare; 5]; 5]" is hard to
// create dynamically (for a newb like me at least).

#[derive(Debug)]
struct BingoBoard {
    board: Vec<BingoSquare>,
    winner: bool,
}

static SQUARE_SIZE: usize = 5;

impl BingoBoard {
    // Given an array with 25 values, we fill the board
    fn new(values: &[usize]) -> BingoBoard {
        let mut board: Vec<BingoSquare> = Vec::new();
        for value in values {
            board.push(BingoSquare {
                value: *value,
                marked: false,
            });
        }

        BingoBoard {
            board,
            winner: false,
        }
    }

    // Given a row and col, return the offset into the vec that corresponds.
    fn vec_offset(row: usize, col: usize) -> usize {
        row * SQUARE_SIZE + col
    }

    // Attempt to mark a value as a match, and return True if a match was found
    // for this board.
    fn mark_match(&mut self, value: usize) -> bool {
        for square in self.board.iter_mut() {
            if square.value == value {
                square.mark();
                return true;
            }
        }

        false
    }

    fn bingo_check(&self) -> bool {
        // Check all rows for a bingo
        let mut result = false;

        for row in 0..SQUARE_SIZE {
            let start_offset = BingoBoard::vec_offset(row, 0);
            let end_offset = start_offset + SQUARE_SIZE;

            result = true;
            for value in start_offset..end_offset {
                if !self.board.get(value).unwrap().marked {
                    result = false;
                    break;
                }
            }

            if result {
                return result;
            }
        }

        // Next, check the columns
        for col in 0..SQUARE_SIZE {
            result = true;
            // With a 5x5 board, this will yield the values 0, 5, 10, 15, 20
            for value in (0..SQUARE_SIZE * SQUARE_SIZE).step_by(SQUARE_SIZE) {
                if !self.board.get(value + col).unwrap().marked {
                    result = false;
                    break;
                }
            }

            if result {
                return result;
            }
        }

        result
    }

    // Board score is the sum of all unmarked squares, multiplied by the last number marked.
    fn board_score(&self, last_number: usize) -> usize {
        let board_sum: usize = self
            .board
            .iter()
            .filter_map(|square| {
                if !square.marked {
                    Some(square.value)
                } else {
                    None
                }
            })
            .sum();

        println!(
            "Found a bingo - number: {:?}, sum: {:?}",
            last_number, board_sum
        );

        board_sum * last_number
    }
}

#[derive(Debug)]
struct BingoSquare {
    value: usize,
    marked: bool,
}

impl BingoSquare {
    fn mark(&mut self) {
        self.marked = true;
    }
}

fn main() {
    let mut parsed_input = parse_by_line(io::stdin().lock());

    let bingo_numbers: Vec<usize> = parsed_input
        .remove(0)
        .split(",")
        .into_iter()
        .map(|v| {
            v.parse::<usize>()
                .expect("Invalid value found - expecting an integer")
        })
        .collect();

    parsed_input.remove(0); // There's an extra newline before the first bingo board starts - skip it.

    let mut all_boards: Vec<BingoBoard> = Vec::new();
    let mut current_board: Vec<usize> = Vec::new();
    for line in parsed_input {
        if line.is_empty() {
            all_boards.push(BingoBoard::new(&current_board));
            current_board = Vec::new();
        } else {
            let mut new_bingo_line: Vec<usize> = line
                .split_whitespace()
                .into_iter()
                .map(|v| {
                    v.parse::<usize>()
                        .expect("Invalid value found - expecting integer!")
                })
                .collect();
            current_board.append(&mut new_bingo_line);
        }
    }

    if current_board.len() > 0 {
        // Input didnt end with an empty line, final board to add
        all_boards.push(BingoBoard::new(&current_board));
    }

    part_1(&bingo_numbers, &mut all_boards);
    part_2(&bingo_numbers, &mut all_boards);
}

fn part_1(bingo_numbers: &Vec<usize>, all_boards: &mut Vec<BingoBoard>) {
    let result = bingo_bango(bingo_numbers, all_boards);
    println!("Bingo Bango: {:?}", result);
}

fn part_2(bingo_numbers: &Vec<usize>, all_boards: &mut Vec<BingoBoard>) {
    let result = bingo_loseo(bingo_numbers, all_boards);
    println!("Bingo Lose-o: {:?}", result);
}

// Return the first winner board score
fn bingo_bango(bingo_numbers: &Vec<usize>, all_boards: &mut Vec<BingoBoard>) -> usize {
    for bingo_number in bingo_numbers {
        for board in all_boards.iter_mut() {
            let match_found = board.mark_match(*bingo_number);
            if match_found && board.bingo_check() {
                return board.board_score(*bingo_number);
            }
        }
    }

    return 0;
}

// Return the score of the board that will win last
fn bingo_loseo(bingo_numbers: &Vec<usize>, all_boards: &mut Vec<BingoBoard>) -> usize {
    let mut non_winning_boards = all_boards.len();

    for bingo_number in bingo_numbers {
        for board in all_boards.iter_mut() {
            if board.winner {
                continue;
            }

            let match_found = board.mark_match(*bingo_number);
            if match_found && board.bingo_check() {
                board.winner = true;
                non_winning_boards -= 1;
            }

            if non_winning_boards == 0 {
                // This was the last board to win - now we can report the score
                return board.board_score(*bingo_number);
            }
        }
    }

    return 0;
}
