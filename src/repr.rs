use std::ops::Not;

#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
pub struct Literal<Atom>(pub(crate) Atom, pub(crate) bool);

impl<Atom> Literal<Atom> {
    pub fn new(atom: Atom) -> Self {
        Self(atom, true)
    }
    pub fn negated(atom: Atom) -> Self {
        Self(atom, false)
    }
    pub fn value(&self) -> bool {
        self.1
    }
    pub fn atom(self) -> Atom {
        self.0
    }
}

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

pub type Clause<Atom> = Vec<Literal<Atom>>;

pub type CNF<Atom> = Vec<Clause<Atom>>;

pub type Assignment<Atom> = Vec<Literal<Atom>>;
