use std::env;
use std::fs::File;
use std::io::Read;
use std::collections::{HashSet, VecDeque};

#[derive(Clone)]
struct Cell {
    value: u8,
    possible: HashSet<u8>,
}

impl Cell {
    fn ignore_and_then_done(&mut self, to_ignore: u8) -> bool {
        if self.is_done() {
            return false;
        }

        self.possible.remove(&to_ignore);
        if self.possiblities() == 1 {
            self.value = self.possible.drain().next().unwrap();
            return true;
        }
        false
    }

    fn possiblities(&self) -> usize {
        self.possible.len()
    }

    fn is_done(&self) -> bool {
        self.value != 0
    }
}

#[derive(Clone)]
struct Sudoku {
    board: Vec<Cell>,
    queue: VecDeque<u8>,
    solutions: Vec<Vec<Cell>>,
}

impl Sudoku {
    fn from(from_board: [u8; 81]) -> Sudoku {
        let full_set: HashSet<u8> = (1..10).collect();

        let mut queue = VecDeque::with_capacity(81);
        let mut board = Vec::with_capacity(81);

        for i in 0..81 {
            board.push(Cell {
                value: from_board[i],
                possible: full_set.clone(),
            });

            if from_board[i] != 0 {
                queue.push_back(i as u8);
            }
        }

        Sudoku {
            board: board,
            queue: queue,
            solutions: vec![],
        }
    }

    fn solve(&mut self) -> bool {
        self.propagate_constraints();
        self.solutions = Sudoku::backtract_solve(self.clone());

        !self.solutions.is_empty()
    }

    fn backtract_solve(state: Sudoku) -> Vec<Vec<Cell>> {
        let mut solutions: Vec<Vec<Cell>> = Vec::new();

        // Find cell with most constraints satisified
        let mut cell = 99;
        let mut cell_poss = 99;
        for i in 0..81 {
            if !state.board[i].is_done() {
                if state.board[i].possiblities() == 0 {
                    return solutions;
                }
                if state.board[i].possiblities() < cell_poss {
                    cell = i;
                    cell_poss = state.board[i].possiblities();
                }
            }
        }

        // No cell? Game is complete
        if cell == 99 {
            if Sudoku::check_valid(&state.board) {
                return vec![state.board];
            }
            return Vec::new();
        }

        let mut try_cell = state.board[cell].clone();

        // Try the various possibilities of the cell.
        for value in try_cell.possible.drain() {
            let mut new_state = state.clone();
            new_state.board[cell].value = value;
            new_state.queue.push_back(cell as u8);
            new_state.propagate_constraints();

            solutions.append(&mut Sudoku::backtract_solve(new_state));
        }

        solutions
    }


    fn propagate_constraints(&mut self) {
        while !self.queue.is_empty() {
            let current_cell = self.queue.pop_front().unwrap();
            let current_value = self.board[current_cell as usize].value;

            //println!("Next Up Cell: [{}] = {}", current_cell, current_value);
            //println!("{:?}", self.queue);

            let row = current_cell / 9 * 9;
            for i in row..row + 9 {
                if self.board[i as usize].ignore_and_then_done(current_value) {
                    self.queue.push_back(i);
                }
            }

            let column = current_cell % 9;
            for i in 0..9 {
                let index = i * 9 + column;
                if self.board[index as usize].ignore_and_then_done(current_value) {
                    self.queue.push_back(index);
                }
            }

            let box_row = current_cell / 9 / 3;
            let box_col = column / 3 * 3;
            let box_start = box_row * 27 + box_col;
            for i in 0..3 {
                for j in 0..3 {
                    let index = box_start + i + j * 9;
                    if self.board[index as usize].ignore_and_then_done(current_value) {
                        self.queue.push_back(index);
                    }
                }
            }
        }
    }

    fn check_valid(board: &Vec<Cell>) -> bool {
        let default_set: Vec<HashSet<u8>> = vec![HashSet::with_capacity(9); 9];

        let mut row_sets = default_set.clone();
        let mut col_sets = default_set.clone();
        let mut box_sets = default_set.clone();

        for i in 0..81 {
            let value = board[i].value;

            let row = i / 9;
            let column = i % 9;
            let boxx = i / 27 * 3 + column / 3;
            if !row_sets[row].insert(value) || !col_sets[column].insert(value)
                || !box_sets[boxx].insert(value)
            {
                return false;
            }
        }

        true
    }

    fn top_left_number(board: &Vec<Cell>) -> u32 {
        board[0].value as u32 * 100 + board[1].value as u32 * 10 + board[2].value as u32
    }

    fn print(board: &Vec<Cell>) {
        println!("----------");
        for i in 0..9 {
            for j in 0..9 {
                let value = board[j + i * 9].value;
                if value == 0 {
                    print!("*");
                } else {
                    print!("{}", value);
                }
            }
            println!("");
        }
        println!("----------");
    }
}


fn process(file: String) {
    let mut file = File::open(file).expect("missing file");
    let mut games = String::new();
    file.read_to_string(&mut games).unwrap();

    let mut games = games.split_whitespace();

    let mut game_number = 1;
    let mut top_three_sum = 0;

    while games.nth(1).is_some() {
        let mut board = [0u8; 81];
        for i in 0..9 {
            let mut n = games.next().unwrap().chars();
            for j in 0..9 {
                board[i * 9 + j] = n.next().unwrap().to_digit(10).unwrap() as u8;
            }
        }



        let mut sudoku = Sudoku::from(board);
        if sudoku.solve() {
            println!("Game Number: {}", game_number);
            game_number += 1;
            for s in sudoku.solutions {
                Sudoku::print(&s);
                top_three_sum += Sudoku::top_left_number(&s);
            }
        } else {
            println!("No solutions :(");
            Sudoku::print(&sudoku.board);
            panic!()
        }
    }
    println!("Top Left Sum : {}", top_three_sum);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        process(args[1].to_owned());
    } else {
        process("p096_sudoku.txt".to_owned());
    }
}
