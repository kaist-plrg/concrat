use std::collections::BTreeSet;

use rustc_mir_dataflow::{fmt::DebugWithContext, lattice::JoinSemiLattice, GenKill};

use crate::util::ExprPath;

pub trait HasBottom {
    fn bottom() -> Self;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MayMutexSet(pub BTreeSet<ExprPath>);

impl HasBottom for MayMutexSet {
    fn bottom() -> Self {
        Self(BTreeSet::new())
    }
}

impl JoinSemiLattice for MayMutexSet {
    fn join(&mut self, other: &Self) -> bool {
        let mut changed = false;
        for p in &other.0 {
            changed |= self.0.insert(p.clone());
        }
        changed
    }
}

impl GenKill<ExprPath> for MayMutexSet {
    fn gen(&mut self, elem: ExprPath) {
        self.0.insert(elem);
    }

    fn kill(&mut self, elem: ExprPath) {
        self.0.remove(&elem);
    }
}

impl<T> DebugWithContext<T> for MayMutexSet {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MustMutexSet {
    All,
    Set(BTreeSet<ExprPath>),
}

impl MustMutexSet {
    pub fn new(set: BTreeSet<ExprPath>) -> Self {
        Self::Set(set)
    }

    pub fn empty() -> Self {
        Self::Set(BTreeSet::new())
    }

    pub fn into_set(self) -> BTreeSet<ExprPath> {
        match self {
            Self::All => panic!(),
            Self::Set(s) => s,
        }
    }

    pub fn retain<F: Fn(&ExprPath) -> bool>(&mut self, f: F) {
        match self {
            Self::All => (),
            Self::Set(s) => s.retain(f),
        }
    }

    pub fn map<F: Fn(ExprPath) -> ExprPath>(self, f: F) -> Self {
        match self {
            Self::All => Self::All,
            Self::Set(mut s) => Self::new(s.drain_filter(|_| true).map(f).collect()),
        }
    }
}

impl HasBottom for MustMutexSet {
    fn bottom() -> Self {
        Self::All
    }
}

impl JoinSemiLattice for MustMutexSet {
    fn join(&mut self, other: &Self) -> bool {
        match (&mut *self, other) {
            (_, Self::All) => false,
            (Self::All, _) => {
                *self = other.clone();
                true
            }
            (Self::Set(s1), Self::Set(s2)) => {
                let len = s1.len();
                s1.retain(|p| s2.contains(p));
                s1.len() < len
            }
        }
    }
}

impl GenKill<ExprPath> for MustMutexSet {
    fn gen(&mut self, elem: ExprPath) {
        match self {
            Self::All => (),
            Self::Set(s) => {
                s.insert(elem);
            }
        }
    }

    fn kill(&mut self, elem: ExprPath) {
        match self {
            Self::All => (),
            Self::Set(s) => {
                s.remove(&elem);
            }
        }
    }
}

impl<T> DebugWithContext<T> for MustMutexSet {}
