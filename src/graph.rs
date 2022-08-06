use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use rustc_data_structures::graph::{scc::Sccs, vec_graph::VecGraph};
use rustc_index::vec::Idx;

#[derive(Clone, Debug)]
pub struct Graph<T> {
    map: HashMap<T, HashSet<T>>,
}

impl<T> Graph<T> {
    pub fn new(map: HashMap<T, HashSet<T>>) -> Self {
        Self { map }
    }

    pub fn into_inner(self) -> HashMap<T, HashSet<T>> {
        self.map
    }
}

impl<T: Clone + Eq + Hash> Graph<T> {
    pub fn transitive_closure(mut self) -> Self {
        let empty = HashSet::new();
        loop {
            let new = self
                .map
                .iter()
                .map(|(k, vs)| {
                    let nvs = vs
                        .iter()
                        .flat_map(|v| self.map.get(v).unwrap_or(&empty).clone())
                        .collect();
                    (k.clone(), vs.union(&nvs).cloned().collect())
                })
                .collect();
            if self.map == new {
                return self;
            }
            self.map = new;
        }
    }
}

pub fn compute_sccs<T: Idx + Ord>(num_nodes: usize, edges: Vec<(T, T)>) -> Sccs<T, T> {
    Sccs::new(&VecGraph::new(num_nodes, edges))
}
