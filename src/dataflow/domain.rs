use std::collections::BTreeSet;

use rustc_mir_dataflow::{fmt::DebugWithContext, lattice::JoinSemiLattice, GenKill};

use crate::util::ExprPath;

pub trait Domain {
    fn lock(&mut self, path: ExprPath);
    fn unlock(&mut self, path: ExprPath);
    fn wait(&mut self, path: ExprPath);

    fn lock_rd(&mut self, path: ExprPath);
    fn lock_wr(&mut self, path: ExprPath);
    fn unlock_rw(&mut self, path: ExprPath);

    fn custom(&mut self, entry: MayMutexSetPair, ret: MustMutexSetTriple);
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MayMutexSetPair {
    pub mutex: MayMutexSet,
    pub rwlock: MayMutexSet,
}

impl MayMutexSetPair {
    pub fn bottom() -> Self {
        Self {
            mutex: MayMutexSet::bottom(),
            rwlock: MayMutexSet::bottom(),
        }
    }

    pub fn map<F: Fn(ExprPath) -> ExprPath>(self, f: F) -> Self {
        Self {
            mutex: self.mutex.map(&f),
            rwlock: self.rwlock.map(&f),
        }
    }
}

impl Domain for MayMutexSetPair {
    fn lock(&mut self, path: ExprPath) {
        self.mutex.kill(path);
    }

    fn unlock(&mut self, path: ExprPath) {
        self.mutex.gen(path);
    }

    fn wait(&mut self, path: ExprPath) {
        self.mutex.gen(path);
    }

    fn lock_rd(&mut self, path: ExprPath) {
        self.rwlock.kill(path);
    }

    fn lock_wr(&mut self, path: ExprPath) {
        self.rwlock.kill(path);
    }

    fn unlock_rw(&mut self, path: ExprPath) {
        self.rwlock.gen(path);
    }

    fn custom(&mut self, entry: MayMutexSetPair, ret: MustMutexSetTriple) {
        fn kill(this: &mut MayMutexSet, ret: MustMutexSet) {
            if let MustMutexSet::Set(ret) = ret {
                this.kill_all(ret);
            } else {
                *this = MayMutexSet::bottom();
            }
        }
        kill(&mut self.mutex, ret.mutex);
        kill(&mut self.rwlock, ret.rdlock);
        kill(&mut self.rwlock, ret.wrlock);

        self.mutex.gen_all(entry.mutex.0);
        self.rwlock.gen_all(entry.rwlock.0);
    }
}

impl JoinSemiLattice for MayMutexSetPair {
    fn join(&mut self, other: &Self) -> bool {
        let b1 = self.mutex.join(&other.mutex);
        let b2 = self.rwlock.join(&other.rwlock);
        b1 || b2
    }
}

impl<T> DebugWithContext<T> for MayMutexSetPair {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MayMutexSet(pub BTreeSet<ExprPath>);

impl MayMutexSet {
    pub fn bottom() -> Self {
        Self(BTreeSet::new())
    }

    pub fn into_vec(mut self) -> Vec<ExprPath> {
        self.0.drain_filter(|_| true).collect()
    }

    pub fn map<F: Fn(ExprPath) -> ExprPath>(mut self, f: F) -> Self {
        Self(self.0.drain_filter(|_| true).map(f).collect())
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
pub struct MustMutexSetTriple {
    pub mutex: MustMutexSet,
    pub rdlock: MustMutexSet,
    pub wrlock: MustMutexSet,
}

impl MustMutexSetTriple {
    pub fn new(entry: MayMutexSetPair) -> Self {
        let MayMutexSetPair { mutex, rwlock } = entry;
        Self {
            mutex: MustMutexSet::new(mutex.0),
            rdlock: MustMutexSet::new(rwlock.0.clone()),
            wrlock: MustMutexSet::new(rwlock.0),
        }
    }

    pub fn bottom() -> Self {
        Self {
            mutex: MustMutexSet::bottom(),
            rdlock: MustMutexSet::bottom(),
            wrlock: MustMutexSet::bottom(),
        }
    }

    pub fn empty() -> Self {
        Self {
            mutex: MustMutexSet::empty(),
            rdlock: MustMutexSet::empty(),
            wrlock: MustMutexSet::empty(),
        }
    }

    pub fn retain<F: Fn(&ExprPath) -> bool>(&mut self, f: F) {
        self.mutex.retain(&f);
        self.rdlock.retain(&f);
        self.wrlock.retain(&f);
    }

    pub fn map<F: Fn(ExprPath) -> ExprPath>(self, f: F) -> Self {
        Self {
            mutex: self.mutex.map(&f),
            rdlock: self.rdlock.map(&f),
            wrlock: self.wrlock.map(&f),
        }
    }

    pub fn append(&mut self, that: Self) {
        self.mutex.append(that.mutex);
        self.rdlock.append(that.rdlock);
        self.wrlock.append(that.wrlock);
    }
}

impl Domain for MustMutexSetTriple {
    fn lock(&mut self, path: ExprPath) {
        self.mutex.gen(path);
    }

    fn unlock(&mut self, path: ExprPath) {
        self.mutex.kill(path);
    }

    fn wait(&mut self, path: ExprPath) {
        self.mutex.gen(path);
    }

    fn lock_rd(&mut self, path: ExprPath) {
        self.rdlock.gen(path);
    }

    fn lock_wr(&mut self, path: ExprPath) {
        self.wrlock.gen(path);
    }

    fn unlock_rw(&mut self, path: ExprPath) {
        self.rdlock.kill(path.clone());
        self.wrlock.kill(path);
    }

    fn custom(&mut self, entry: MayMutexSetPair, ret: MustMutexSetTriple) {
        self.mutex.kill_all(entry.mutex.0);
        self.rdlock.kill_all(entry.rwlock.0.clone());
        self.wrlock.kill_all(entry.rwlock.0);

        fn gen(this: &mut MustMutexSet, ret: MustMutexSet) {
            if let MustMutexSet::Set(ret) = ret {
                this.gen_all(ret);
            } else {
                *this = MustMutexSet::bottom();
            }
        }
        gen(&mut self.mutex, ret.mutex);
        gen(&mut self.rdlock, ret.rdlock);
        gen(&mut self.wrlock, ret.wrlock);
    }
}

impl JoinSemiLattice for MustMutexSetTriple {
    fn join(&mut self, other: &Self) -> bool {
        let b1 = self.mutex.join(&other.mutex);
        let b2 = self.rdlock.join(&other.rdlock);
        let b3 = self.wrlock.join(&other.wrlock);
        b1 || b2 || b3
    }
}

impl<T> DebugWithContext<T> for MustMutexSetTriple {}

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

    pub fn bottom() -> Self {
        Self::All
    }

    pub fn into_set(self) -> BTreeSet<ExprPath> {
        match self {
            Self::All => panic!(),
            Self::Set(s) => s,
        }
    }

    pub fn into_vec(self) -> Vec<ExprPath> {
        match self {
            Self::All => panic!(),
            Self::Set(mut s) => s.drain_filter(|_| true).collect(),
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

    pub fn append(&mut self, that: Self) {
        match that {
            Self::All => *self = Self::All,
            Self::Set(s) => self.gen_all(s),
        }
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
