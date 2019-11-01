use std::ops::Not;

#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
pub(crate) struct Literal<Atom>(pub Atom, pub bool);

impl<Atom> Not for Literal<Atom>
where
    Atom: Clone,
{
    type Output = Literal<Atom>;
    fn not(self) -> Self::Output {
        Literal(self.0.clone(), !self.1)
    }
}

impl<Atom> Not for &Literal<Atom>
where
    Atom: Clone,
{
    type Output = Literal<Atom>;
    fn not(self) -> Self::Output {
        Literal(self.0.clone(), !self.1)
    }
}

pub(crate) type Clause<Atom> = Vec<Literal<Atom>>;

pub(crate) type CNF<Atom> = Vec<Clause<Atom>>;

pub(crate) type Assignment<Atom> = Vec<Literal<Atom>>;
