use crate::repr::*;
use std::collections::HashSet;
use std::hash::Hash;

fn propagate_units<Atom>(cnf: &CNF<Atom>) -> Option<(CNF<Atom>, Assignment<Atom>)>
where
    Atom: Hash + PartialEq + Eq + Clone,
{
    let mut seen_atoms = HashSet::new();
    let assignment = cnf
        .iter()
        .filter(|clause| clause.len() == 1 && seen_atoms.insert(clause[0].0.clone()))
        .map(|clause| clause[0].clone())
        .collect::<Assignment<_>>();
    propagate_literals(cnf, &assignment).map(|cnf| (cnf, assignment))
}

fn propagate_literals<Atom>(cnf: &CNF<Atom>, assignment: &Assignment<Atom>) -> Option<CNF<Atom>>
where
    Atom: PartialEq + Eq + Clone,
{
    let mut updated_cnf = Vec::new();
    for clause in cnf.iter() {
        if !clause.iter().any(|literal| assignment.contains(literal)) {
            let clause: Clause<_> = clause
                .iter()
                .cloned()
                .filter(|literal| !assignment.contains(&!literal))
                .collect();
            if clause.is_empty() {
                return None;
            }
            updated_cnf.push(clause);
        }
    }
    Some(updated_cnf)
}

fn eliminate_pure_literals<Atom>(cnf: &CNF<Atom>) -> (CNF<Atom>, Assignment<Atom>)
where
    Atom: Hash + PartialEq + Eq + Clone,
{
    let mut seen_atoms = HashSet::new();
    let mut potentially_pure = HashSet::new();
    for clause in cnf.iter() {
        for literal in clause.iter() {
            if seen_atoms.insert(literal.0.clone()) {
                potentially_pure.insert(literal);
            } else {
                potentially_pure.remove(&!literal);
            }
        }
    }
    (
        cnf.iter()
            .filter(|clause| {
                !clause
                    .iter()
                    .any(|literal| potentially_pure.contains(&literal))
            })
            .cloned()
            .collect(),
        potentially_pure.into_iter().cloned().collect(),
    )
}

pub(crate) fn solve<Atom>(cnf: &CNF<Atom>) -> Option<Assignment<Atom>>
where
    Atom: Hash + PartialEq + Eq + Clone,
{
    if let Some((cnf, mut assignment)) = propagate_units(cnf) {
        let (cnf, mut assignment2) = eliminate_pure_literals(&cnf);
        assignment.append(&mut assignment2);
        if cnf.is_empty() {
            return Some(assignment);
        }
        let mut assignment_true = vec![cnf[0][0].clone()];
        if let Some(cnf) = propagate_literals(&cnf, &assignment_true) {
            if let Some(mut solution) = solve(&cnf) {
                assignment.append(&mut assignment_true);
                assignment.append(&mut solution);
                return Some(assignment);
            }
        }
        let mut assignment_false = vec![!&cnf[0][0]];
        if let Some(cnf) = propagate_literals(&cnf, &assignment_false) {
            return solve(&cnf).map(|mut solution| {
                assignment.append(&mut assignment_false);
                assignment.append(&mut solution);
                assignment
            });
        }
    }
    None
}

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
                        Literal(SudokuCell::new(row, col, val), false),
                        Literal(SudokuCell::new(other, col, val), false),
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
                        Literal(SudokuCell::new(row, col, val), false),
                        Literal(SudokuCell::new(row, other, val), false),
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
                            Literal(SudokuCell::new(r1, c1, val), false),
                            Literal(SudokuCell::new(r2, c2, val), false),
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
        for (col, cell) in (0..9).zip(line.chars()) {
            match cell {
                '1'..='9' => cnf.push(vec![Literal(
                    SudokuCell::new(row, col, (cell as u8) - ('0' as u8)),
                    true,
                )]),
                _ => cnf.push(
                    (1..=9)
                        .map(|val| Literal(SudokuCell::new(row, col, val), true))
                        .collect(),
                ),
            }
        }
    }
    let solution = solve(&cnf)?;
    let mut grid = [[' '; 9]; 9];
    for (row, col, val) in solution
        .iter()
        .filter(|literal| literal.1)
        .map(|literal| literal.0.as_tuple())
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
    fn find_trivial_contradiction() {
        let cnf = vec![
            vec![Literal("a".to_string(), true)],
            vec![Literal("a".to_string(), false)],
        ];
        assert!(solve(&cnf).is_none());
    }

    #[test]
    fn find_trivial_tautology() {
        let cnf = vec![vec![
            Literal("a".to_string(), true),
            Literal("a".to_string(), false),
        ]];
        assert!(solve(&cnf).is_some());
    }

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
