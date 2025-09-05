use tailcall::tailcall;

#[derive(Copy, Clone)]
pub struct Sudoku(pub(crate) [[u8; 9]; 9]);

impl Sudoku {
    fn push_value(&mut self, index: [usize; 2], value: u8) {
        self.0[index[0]][index[1]] = value
    }
}

#[derive(Copy, Clone)]
struct SudokuInfo {
    sudoku: Sudoku,
    index: [usize; 2],
    value: u8
}

impl SudokuInfo {
    fn new(sudoku: Sudoku, index: [usize; 2], value: u8) -> SudokuInfo {
        SudokuInfo { sudoku, index, value }
    }

    fn with_value(&self, value: u8) -> SudokuInfo {
        SudokuInfo { sudoku: self.sudoku, index: self.index, value }
    }

    fn push_value(&mut self) {
        let mut sudoku = self.sudoku;
        sudoku.push_value(self.index, self.value);
        self.sudoku = sudoku
    }
}

pub fn start_resolution(mut args: std::env::Args) -> Result<Sudoku, &'static str> {
    match args.nth(1) {
        Some(sudoku_code) => {
            let sudoku = convert_sudoku(sudoku_code)?;
            solve_sudoku(sudoku)
        },
        None => Err(
            "Argument not found: try with: \n\t\tsudoku_solver [sudoku code]"
        )
    }
}

fn convert_sudoku(sudoku_code: String) -> Result<Sudoku, &'static str> {
    if sudoku_code.len() != 81 {
        Err("Invalid sudoku length: try with a number with 81 digits")
    } else {
        let mut sudoku_converted: [[u8; 9]; 9] = [[0; 9]; 9];

        for (i, c) in sudoku_code.chars().enumerate() {
            sudoku_converted[i / 9][i % 9] =
                match c.to_digit(10) {
                    Some(number) => Ok(number),
                    None => Err("Invalid sudoku type: try with a decimal system integer")
                }? as u8
        }

        Ok(Sudoku(sudoku_converted))
    }
}

fn solve_sudoku(sudoku: Sudoku) -> Result<Sudoku, &'static str> {
    #[tailcall]
    fn solve(mut sudoku_list: Vec<SudokuInfo>, last_sudoku: SudokuInfo) -> Result<Sudoku, &'static str> {
        if last_sudoku.sudoku.0[last_sudoku.index[0]][last_sudoku.index[1]] == 0 {
            if last_sudoku.value >= 9 {
                match sudoku_list.pop() {
                    Some(sudoku_info) => {
                        let mut sudoku_updated = SudokuInfo::new(sudoku_info.sudoku, sudoku_info.index, 0);
                        sudoku_updated.push_value();
                        solve(sudoku_list, sudoku_updated.with_value(sudoku_info.value + 1))
                    },
                    None => Err("This sudoku has no solution")
                }
            } else {
                let mut new_sudoku = last_sudoku.with_value(last_sudoku.value + 1);
                if check_safeness(&new_sudoku) {
                    new_sudoku.push_value();
                    solve(sudoku_list.into_iter().chain(std::iter::once(last_sudoku)).collect(), new_sudoku)
                } else {
                    solve(sudoku_list, new_sudoku)
                }
            }
        } else {
            if last_sudoku.index[0] == 8 {
                if last_sudoku.index[1] == 8 {
                    Ok(last_sudoku.sudoku)
                } else {
                    solve(sudoku_list, SudokuInfo::new(last_sudoku.sudoku, [0, last_sudoku.index[1] + 1], 0))
                }
            } else {
                solve(sudoku_list, SudokuInfo::new(last_sudoku.sudoku, [last_sudoku.index[0] + 1, last_sudoku.index[1]], 0))
            }
        }
    }

    solve(vec![], SudokuInfo::new(sudoku, [0, 0], 0))
}

fn check_safeness(sudoku_info: &SudokuInfo) -> bool {
    for x in 0..9 {
        if x != sudoku_info.index[0] {
            if sudoku_info.sudoku.0[x][sudoku_info.index[1]] == sudoku_info.value {
                return false
            }
        }
    }

    for y in 0..9 {
        if y != sudoku_info.index[1] {
            if sudoku_info.sudoku.0[sudoku_info.index[0]][y] == sudoku_info.value {
                return false
            }
        }
    }

    for x in 0..3 {
        for y in 0..3 {
            if x != sudoku_info.index[0] % 3 && y != sudoku_info.index[1] % 3 {
                if sudoku_info.sudoku.0[x + sudoku_info.index[0] / 3 * 3][y + sudoku_info.index[1] / 3 * 3] == sudoku_info.value {
                    return false
                }
            }
        }
    }

    true
}