/* Sudoku Solver in Rust
 *
 * Based off of my own original C code, which can be found at
 * https://zhaba.dev/Sudoku-Solver-in-C
 *
 */

// Puzzle isn't being read correctly and causing errors
// fix index out of bounds error if possible

use std::fs;

const DIM: usize = 9;
const SUBGRID_DIM: usize = 3;
const ROWLEN: usize = 9;


struct Puzzle {
    values:  [[usize; DIM]; DIM],
    isFixed: [[ bool; DIM]; DIM],
    colHasVal: [[ bool; DIM ]; DIM ],
    rowHasVal: [[ bool; DIM]; DIM],
    subgridHasVal: [[[bool; DIM ]; SUBGRID_DIM]; SUBGRID_DIM],
}


impl Default for Puzzle {
    fn default () -> Puzzle{
        Puzzle{
            values:  [[0; DIM]; DIM],
            isFixed: [[ false; DIM]; DIM],
            colHasVal: [[ false; DIM ]; DIM ],
            rowHasVal: [[ false; DIM]; DIM],
            subgridHasVal: [[[false; DIM ]; SUBGRID_DIM]; SUBGRID_DIM],
        }
    }
}

fn load_puzzle(file_name: &str) -> Puzzle {
    let mut newPuzzle = Puzzle::default();
    let mut real_idx: usize = 0;
    let mut col: usize = 0;
    let mut row: usize = 0;

    let mut contents = fs::read_to_string(file_name)
        .expect("An error occurred while reading the file");


    contents = contents.trim().to_string();

    for (i, c) in contents.chars().enumerate() {
        if !c.is_whitespace(){
            let val: usize = c.to_digit(10).unwrap() as usize;
            row = real_idx / DIM;
            col = real_idx % DIM;

            newPuzzle.values[row][col] = val;

            if val != 0 {
                newPuzzle.isFixed[row][col] = true;
                newPuzzle.colHasVal[col][val] = true;
                newPuzzle.rowHasVal[row ][val ] = true;
                newPuzzle.subgridHasVal[row  / SUBGRID_DIM][col / SUBGRID_DIM][val] = true;
            }
            real_idx += 1;
        }
    }

    return newPuzzle;
}
impl Puzzle{
    fn is_safe(&mut self, val: usize, row: usize, col: usize) -> bool {
        return !(self.rowHasVal[row][val])
                && !(self.colHasVal[col][val])
                && !(self.subgridHasVal[row / 3][col / 3][val])
                && !(self.isFixed[row][col]);
    }

    fn place_val(&mut self, val: usize, row: usize, col: usize){
        self.values[row][col] = val;
        self.subgridHasVal[row / SUBGRID_DIM][col / SUBGRID_DIM][val] = true;
        self.rowHasVal[row][val] = true;
        self.colHasVal[col][val] = true;
    }

    fn remove_val(&mut self, val: usize, row: usize, col: usize){
        self.values[row][col] = 0;
        self.subgridHasVal[row / SUBGRID_DIM][col / SUBGRID_DIM][val] = false;
        self.rowHasVal[row][val] = false;
        self.colHasVal[col][val] = false;
    }

    pub fn solve(&mut self, n: usize) -> bool {
        if n == 81 {
            return true;
        }
        let row: usize = n / 9;
        let col: usize = n % 9;

        if self.isFixed[row][col] {
            if self.solve(n + 1) {
                return true;
            }
        }

        for val in 1..9 {
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



fn main(){
    let mut puzzle = load_puzzle("puzzle_test.txt");
    puzzle.solve(0);
    puzzle.print_puzzle();

}
