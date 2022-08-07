use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use rustc_data_structures::graph::{scc::Sccs, vec_graph::VecGraph};
use rustc_index::vec::Idx;

pub fn transitive_closure<T: Clone + Eq + Hash>(
    mut map: HashMap<T, HashSet<T>>,
) -> HashMap<T, HashSet<T>> {
    let empty = HashSet::new();
    loop {
        let new = map
            .iter()
            .map(|(k, vs)| {
                let nvs = vs
                    .iter()
                    .flat_map(|v| map.get(v).unwrap_or(&empty).clone())
                    .collect();
                (k.clone(), vs.union(&nvs).cloned().collect())
            })
            .collect();
        if map == new {
            return map;
        }
        map = new;
    }
}

pub fn inverse<T: Clone + Eq + Hash>(map: &HashMap<T, HashSet<T>>) -> HashMap<T, HashSet<T>> {
    let mut inv: HashMap<_, HashSet<_>> = HashMap::new();
    for node in map.keys() {
        inv.insert(node.clone(), HashSet::new());
    }
    for (node, succs) in map {
        for succ in succs {
            inv.get_mut(succ).unwrap().insert(node.clone());
        }
    }
    inv
}

pub fn reverse_post_order<T: Clone + Eq + Hash>(
    map: &HashMap<T, HashSet<T>>,
    mut inv_map: HashMap<T, HashSet<T>>,
) -> Vec<Vec<T>> {
    let mut res = vec![];
    while let Some((root, _)) = inv_map.iter().min_by_key(|(_, preds)| preds.len()) {
        let root = root.clone();
        let mut stack = vec![];
        let mut v = vec![];
        let mut reached = HashSet::new();
        stack.push(root.clone());
        reached.insert(root);
        while let Some(node) = stack.last() {
            let succs: Vec<_> = map
                .get(node)
                .unwrap()
                .iter()
                .filter(|n| !reached.contains(n))
                .cloned()
                .collect();
            if succs.is_empty() {
                v.push(stack.pop().unwrap());
            } else {
                stack.append(&mut succs.clone());
                for succ in succs {
                    reached.insert(succ);
                }
            }
        }
        for node in &v {
            inv_map.remove(node);
        }
        v.reverse();
        res.push(v);
    }
    res
}

pub fn compute_sccs<T: Idx + Ord>(num_nodes: usize, edges: Vec<(T, T)>) -> Sccs<T, T> {
    Sccs::new(&VecGraph::new(num_nodes, edges))
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    #[test]
    fn test1() {
        let mut graph = HashMap::new();
        graph.insert(1, HashSet::from([2]));
        graph.insert(2, HashSet::from([3]));
        graph.insert(3, HashSet::from([4, 5]));
        graph.insert(4, HashSet::from([6]));
        graph.insert(5, HashSet::from([6]));
        graph.insert(6, HashSet::from([]));

        let closure = super::transitive_closure(graph.clone());
        assert_eq!(closure.get(&1).unwrap(), &HashSet::from([2, 3, 4, 5, 6]));
        assert_eq!(closure.get(&2).unwrap(), &HashSet::from([3, 4, 5, 6]));
        assert_eq!(closure.get(&3).unwrap(), &HashSet::from([4, 5, 6]));
        assert_eq!(closure.get(&4).unwrap(), &HashSet::from([6]));
        assert_eq!(closure.get(&5).unwrap(), &HashSet::from([6]));
        assert_eq!(closure.get(&6).unwrap(), &HashSet::from([]));

        let inv_graph = super::inverse(&graph);
        assert_eq!(inv_graph.get(&1).unwrap(), &HashSet::from([]));
        assert_eq!(inv_graph.get(&2).unwrap(), &HashSet::from([1]));
        assert_eq!(inv_graph.get(&3).unwrap(), &HashSet::from([2]));
        assert_eq!(inv_graph.get(&4).unwrap(), &HashSet::from([3]));
        assert_eq!(inv_graph.get(&5).unwrap(), &HashSet::from([3]));
        assert_eq!(inv_graph.get(&6).unwrap(), &HashSet::from([4, 5]));

        let mut rpo = super::reverse_post_order(&graph, inv_graph);
        assert_eq!(rpo.len(), 1);
        let v = rpo.pop().unwrap();
        assert!(v == vec![1, 2, 3, 4, 5, 6] || v == vec![1, 2, 3, 5, 4, 6]);
    }

    #[test]
    fn test2() {
        let mut graph = HashMap::new();
        graph.insert(1, HashSet::from([1, 2]));
        graph.insert(2, HashSet::from([2, 1]));

        let closure = super::transitive_closure(graph.clone());
        assert_eq!(graph, closure);

        let inv_graph = super::inverse(&graph);
        assert_eq!(graph, inv_graph);

        let mut rpo = super::reverse_post_order(&graph, inv_graph);
        assert_eq!(rpo.len(), 1);
        let v = rpo.pop().unwrap();
        assert!(v == vec![1, 2] || v == vec![1, 2]);
    }

    #[test]
    fn test3() {
        let mut graph = HashMap::new();
        graph.insert(1, HashSet::from([]));
        graph.insert(2, HashSet::from([]));

        let closure = super::transitive_closure(graph.clone());
        assert_eq!(graph, closure);

        let inv_graph = super::inverse(&graph);
        assert_eq!(graph, inv_graph);

        let rpo = super::reverse_post_order(&graph, inv_graph);
        assert!(rpo == vec![vec![1], vec![2]] || rpo == vec![vec![2], vec![1]]);
    }
}
