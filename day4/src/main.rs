//include fs
use std::fs; 
use std::convert::TryInto;


fn main() {
    part_two();
}

fn part_two() {
    //read single line from file
    let contents = fs::read_to_string("input.txt").expect("Error reading file");
    //split into iterator
    let lines: Vec<&str> = contents.split("\n").collect();
    //split first line into i32 comma separated
    let called_numbers: Vec<i32> = lines[0].split(",").map(|x| x.parse::<i32>().unwrap()).collect();



    let mut boards: Vec<BingoBoard> = Vec::new();
    for i in (2..lines.len()).step_by(6) {
        let mut board: [[i32; 5]; 5] = [[-1; 5]; 5];
        //read line[i] into array of 5 ints
        for j in 0..5 {
            let row: Vec<_> = lines[i+j].split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
            board[j] = row.try_into().unwrap();
        }
        boards.push(BingoBoard::new(board));
    }

    let len: i32 = boards.len() as i32;
    let mut win_counter = 0;
    'outer: for i in called_numbers {
        for board in boards.iter_mut() {
            if board.mark(i){
                if board.check_bingo() && !board.has_won {
                    win_counter += 1;
                    board.has_won = true;
                    board.print();
                    if win_counter == len {
                        println!("{}", board.sum_of_unmarked());
                        println!("{}", i);
                        println!("{}", board.sum_of_unmarked()*i);
                        break 'outer;
                    }
                    
                }
            }
        }
    }
}

fn part_one() {
    //read single line from file
    let contents = fs::read_to_string("input.txt").expect("Error reading file");
    //split into iterator
    let lines: Vec<&str> = contents.split("\n").collect();
    //split first line into i32 comma separated
    let called_numbers: Vec<i32> = lines[0].split(",").map(|x| x.parse::<i32>().unwrap()).collect();



    let mut boards: Vec<BingoBoard> = Vec::new();
    for i in (2..lines.len()).step_by(6) {
        let mut board: [[i32; 5]; 5] = [[-1; 5]; 5];
        //read line[i] into array of 5 ints
        for j in 0..5 {
            let row: Vec<_> = lines[i+j].split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
            board[j] = row.try_into().unwrap();
        }
        boards.push(BingoBoard::new(board));
    }

    
   
    'outer: for i in called_numbers{
        for board in boards.iter_mut() {
            if board.mark(i){
                if board.check_bingo(){
                    println!("{}", board.sum_of_unmarked()*i);
                    break 'outer;
                }
            }
        }
    }
}


//struct representing a 5x5 bingo board of integers
#[derive(Copy, Clone)]
struct BingoBoard {
    board: [[i32; 5]; 5],
    is_called: [[bool; 5]; 5],
    has_won: bool,
}


impl BingoBoard {
    //constructor
    fn new(board: [[i32; 5]; 5]) -> BingoBoard {
        let is_called = [[false; 5]; 5];
        BingoBoard {
            board,
            is_called,
            has_won: false,
        }
    }

    //check if board has 5 in a row, column, or diagonal
    fn check_bingo(&self) -> bool {
        let mut bingo = false;
        for i in 0..5 {
            bingo = self.check_row(i) || bingo;
            bingo = self.check_column(i) || bingo;
        }
        bingo = self.check_diagonal_1() || bingo; 
        bingo = self.check_diagonal_2() || bingo;
        
        bingo
    }

    //check if row has bingo
    fn check_row(&self, row: usize) -> bool {
        let mut bingo = true;
        for i in 0..5 {
            bingo = self.is_called[row][i] && bingo;
        }
        bingo
    }

    //check if column has bingo
    fn check_column(&self, column: usize) -> bool {
        let mut bingo = true;
        for i in 0..5 {
            bingo = self.is_called[i][column] && bingo;
        }
        bingo
    }

    //check if diagonal has bingo
    fn check_diagonal_1(&self) -> bool {
        let mut bingo = true;
        for i in 0..5 {
            bingo = self.is_called[i][i] && bingo;
        }
        bingo
    }

    //check if diagonal has bingo
    fn check_diagonal_2(&self) -> bool {
        let mut bingo = true;
        for i in 0..5 {
            bingo = self.is_called[i][4-i] && bingo;
        }
        bingo
    }


    

    //mark a number on the board if exists
    fn mark(&mut self, number: i32) -> bool {
        for i in 0..5 {
            for j in 0..5 {
                if self.board[i][j] == number {
                    self.is_called[i][j] = true;
                    return true;
                }
            }
        }
        false
    }


    //prints the board
    fn print(&self) {
        for i in 0..5 {
            for j in 0..5 {
                print!("{} ", self.board[i][j]);
            }
            println!();
        }
    }

    // return sum of all unmarked numbers 
    fn sum_of_unmarked(&self) -> i32 {
        let mut sum = 0;
        for i in 0..5 {
            for j in 0..5 {
                if !self.is_called[i][j] {
                    sum += self.board[i][j];
                }
            }
        }
        sum
    }
    
}
