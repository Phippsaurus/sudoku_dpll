use dpll::solve::*;
#[derive(Clone, PartialEq, Eq, Hash)]
struct SudokuCell {
    row_col: u8,
    val: u8,
}

impl std::fmt::Debug for SudokuCell {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "({}{}={})",
            (self.row_col / 9 + ('a' as u8)) as char,
            self.row_col % 9,
            self.val
        )
    }
}

impl SudokuCell {
    fn new(row: u8, col: u8, val: u8) -> Self {
        Self {
            row_col: row * 9 + col,
            val,
        }
    }

    fn as_tuple(&self) -> (usize, usize, u8) {
        (
            self.row_col as usize / 9,
            self.row_col as usize % 9,
            self.val,
        )
    }
}

fn sudoku_constraints() -> CNF<SudokuCell> {
    let mut cnf = vec![];

    for row in 0..8 {
        for other in row + 1..9 {
            for col in 0..9 {
                for val in 1..=9 {
                    cnf.push(vec![
                        Literal::negated(SudokuCell::new(row, col, val)),
                        Literal::negated(SudokuCell::new(other, col, val)),
                    ]);
                }
            }
        }
    }

    for row in 0..9 {
        for col in 0..8 {
            for other in col + 1..9 {
                for val in 1..=9 {
                    cnf.push(vec![
                        Literal::negated(SudokuCell::new(row, col, val)),
                        Literal::negated(SudokuCell::new(row, other, val)),
                    ]);
                }
            }
        }
    }

    for horizontal in 0..3 {
        for vertical in 0..3 {
            let block_coord = |coord: u8| (3 * horizontal + coord / 3, 3 * vertical + coord % 3);
            for cell in 0..8 {
                let (r1, c1) = block_coord(cell);
                for other in cell + 1..9 {
                    for val in 1..=9 {
                        let (r2, c2) = block_coord(other);
                        cnf.push(vec![
                            Literal::negated(SudokuCell::new(r1, c1, val)),
                            Literal::negated(SudokuCell::new(r2, c2, val)),
                        ]);
                    }
                }
            }
        }
    }

    cnf
}

pub(crate) fn solve_sudoku(sudoku: &str) -> Option<String> {
    let mut cnf = sudoku_constraints();
    for (row, line) in (0..9).zip(sudoku.lines()) {
        for (col, cell) in (0..9).zip(line.chars().chain(std::iter::repeat(' '))) {
            match cell {
                '1'..='9' => cnf.push(vec![Literal::new(SudokuCell::new(
                    row,
                    col,
                    (cell as u8) - ('0' as u8),
                ))]),
                _ => cnf.push(
                    (1..=9)
                        .map(|val| Literal::new(SudokuCell::new(row, col, val)))
                        .collect(),
                ),
            }
        }
    }
    let solution = solve(&cnf)?;
    let mut grid = [[' '; 9]; 9];
    for (row, col, val) in solution
        .iter()
        .filter(|literal| literal.value())
        .cloned()
        .map(|literal| literal.atom().as_tuple())
    {
        grid[row][col] = (val + ('0' as u8)) as char;
    }
    let mut solution = String::new();
    solution.reserve(90);
    for row in grid.iter() {
        for cell in row.iter() {
            solution.push(*cell);
        }
        solution.push('\n');
    }
    Some(solution)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn solve_2x2_sudoku() {
        // +-+-+
        // |2| |
        // +-+-+
        // |1| |
        // +-+-+
        let cnf = vec![
            vec![
                Literal(SudokuCell::new(0, 0, 1), false),
                Literal(SudokuCell::new(0, 1, 1), false),
            ],
            vec![
                Literal(SudokuCell::new(1, 0, 1), false),
                Literal(SudokuCell::new(1, 1, 1), false),
            ],
            vec![
                Literal(SudokuCell::new(0, 0, 1), false),
                Literal(SudokuCell::new(1, 0, 1), false),
            ],
            vec![
                Literal(SudokuCell::new(0, 1, 1), false),
                Literal(SudokuCell::new(1, 1, 1), false),
            ],
            vec![
                Literal(SudokuCell::new(0, 0, 2), false),
                Literal(SudokuCell::new(0, 1, 2), false),
            ],
            vec![
                Literal(SudokuCell::new(1, 0, 2), false),
                Literal(SudokuCell::new(1, 1, 2), false),
            ],
            vec![
                Literal(SudokuCell::new(0, 0, 2), false),
                Literal(SudokuCell::new(1, 0, 2), false),
            ],
            vec![
                Literal(SudokuCell::new(0, 1, 2), false),
                Literal(SudokuCell::new(1, 1, 2), false),
            ],
            vec![Literal(SudokuCell::new(0, 0, 2), true)],
            vec![Literal(SudokuCell::new(1, 0, 1), true)],
            vec![
                Literal(SudokuCell::new(0, 1, 1), true),
                Literal(SudokuCell::new(0, 1, 2), true),
            ],
            vec![
                Literal(SudokuCell::new(1, 1, 1), true),
                Literal(SudokuCell::new(1, 1, 2), true),
            ],
        ];
        let solution = solve(&cnf);
        assert!(solution.is_some());
    }

    #[test]
    fn solve_already_solved_sudoku() {
        let sudoku_grid = r###"324569187
716824935
958713264
642381579
875496312
139257648
283145796
597638421
461972853
"###;
        let solution = solve_sudoku(sudoku_grid);
        assert_eq!(Some(sudoku_grid.to_string()), solution);
    }

    #[test]
    fn solve_simple_sudoku() {
        let solution = solve_sudoku(
            r###"53__7____
6__195___
_98____6_
8___6___3
4__8_3__1
7___2___6
_6____28_
___419__5
____8__79
"###,
        );
        assert!(solution.is_some());
    }
}
