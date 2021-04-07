/* Sudoku Solver in Rust
 *
 * Based off of my own original C code, which can be found at
 * https://zhaba.dev/Sudoku-Solver-in-C
 *
 */

use std::env;
use std::fs;

const DIM: usize = 9;
const SUBGRID_DIM: usize = 3;
const VAL_DIM: usize = 10;


struct Puzzle {
    values:  [[usize; VAL_DIM]; DIM],
    is_fixed: [[ bool; DIM]; DIM],
    col_has_val: [[ bool; VAL_DIM ]; DIM ],
    row_has_val: [[ bool; VAL_DIM]; DIM],
    subgrid_has_val: [[[bool; VAL_DIM ]; SUBGRID_DIM]; SUBGRID_DIM],
}

impl Default for Puzzle {
    fn default () -> Puzzle{
        Puzzle{
            values:  [[0; VAL_DIM]; DIM],
            is_fixed: [[ false; DIM]; DIM],
            col_has_val: [[ false; VAL_DIM ]; DIM ],
            row_has_val: [[ false; VAL_DIM]; DIM],
            subgrid_has_val: [[[false; VAL_DIM ]; SUBGRID_DIM]; SUBGRID_DIM],
        }
    }
}

impl Puzzle{
    fn is_safe(&mut self, val: usize, row: usize, col: usize) -> bool {
        return !(self.row_has_val[row][val])
                && !(self.col_has_val[col][val])
                && !(self.subgrid_has_val[row / 3][col / 3][val])
                && !(self.is_fixed[row][col]);
    }

    fn place_val(&mut self, val: usize, row: usize, col: usize){
        self.values[row][col] = val;
        self.subgrid_has_val[row / SUBGRID_DIM][col / SUBGRID_DIM][val] = true;
        self.row_has_val[row][val] = true;
        self.col_has_val[col][val] = true;
    }

    fn remove_val(&mut self, val: usize, row: usize, col: usize){
        self.values[row][col] = 0;
        self.subgrid_has_val[row / SUBGRID_DIM][col / SUBGRID_DIM][val] = false;
        self.row_has_val[row][val] = false;
        self.col_has_val[col][val] = false;
    }

    pub fn solve(&mut self, n: usize) -> bool {
        if n == 81 {
            return true;
        }

        let row: usize = n / 9;
        let col: usize = n % 9;

        if self.is_fixed[row][col] {
            if self.solve(n + 1) {
                return true;
            }
        }
        // nobody told me this is the same as i = 1; i < 10
        for val in 1..10 {

            if self.is_safe(val, row, col) {

                self.place_val(val, row, col);

                if self.solve(n + 1) {
                    return true;
                }

                self.remove_val(val, row, col);

            }
        }
        return false;

    }

    pub fn print_puzzle(&mut self){
        for i in 0..9 {
            for j in 0..9 {
                print!("{} ",self.values[i][j]);
            }
            println!("");
        }
    }
}

fn load_puzzle(file_name: &str) -> Puzzle {
    let mut new_puzzle = Puzzle::default();
    let mut real_idx: usize = 0;
    let mut col: usize;
    let mut row: usize;

    let mut contents = fs::read_to_string(file_name)
        .expect("An error occurred while reading the file");


    contents = contents.trim().to_string();

    for c in contents.chars() {
        if !c.is_whitespace(){
            let val: usize = c.to_digit(10).unwrap() as usize;
            row = real_idx / DIM;
            col = real_idx % DIM;

            new_puzzle.values[row][col] = val;

            if val != 0 {
                new_puzzle.is_fixed[row][col] = true;
                new_puzzle.col_has_val[col][val] = true;
                new_puzzle.row_has_val[row ][val ] = true;
                new_puzzle.subgrid_has_val[row  / SUBGRID_DIM][col / SUBGRID_DIM][val] = true;
            }
            real_idx += 1;
        }
    }

    return new_puzzle;
}

fn main(){

    let args: Vec<String> = env::args().collect();
    let mut puzzle = load_puzzle(&args[1]);
    println!("Original Puzzle: ");
    puzzle.print_puzzle();

    puzzle.solve(0);

    println!("\nSolution:");
    puzzle.print_puzzle();
}
