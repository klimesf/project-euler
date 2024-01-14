use std::fs;

pub(crate) fn e96() {
    println!("{}", calc(fs::read_to_string("input/e96/sudoku.txt").unwrap()))
}

fn calc(input: String) -> u32 {
    let mut grids = vec!();
    let mut current_grid = vec!();
    for line in input.lines() {
        if line.starts_with("Grid") {
            if current_grid.len() > 0 {
                grids.push(current_grid.clone());
                current_grid.clear();
            }
            continue
        }
        let row: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
        current_grid.push(row);
    }
    grids.push(current_grid.clone());
    current_grid.clear();

    let mut ans = 0;
    for grid in grids {
        let solution = solve_sudoku(grid);
        ans += solution[0][0] * 100 + solution[0][1] * 10 + solution[0][2];
    }
    ans
}

fn solve_sudoku(sudoku: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    // TODO: backtracking sudoku solver
    sudoku
}

#[cfg(test)]
mod e96_tests {
    use std::fs;
    use crate::e96::{calc};

    #[test]
    fn calc_works() {
        assert_eq!(0, calc(fs::read_to_string("input/e96/sudoku.txt").unwrap()));
    }
}
