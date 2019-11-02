pub use crate::repr::*;
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

pub fn solve<Atom>(cnf: &CNF<Atom>) -> Option<Assignment<Atom>>
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
}
