use std::collections::{BTreeMap, BTreeSet};

use rustc_data_structures::graph::{scc::Sccs, vec_graph::VecGraph};
use rustc_index::vec::Idx;

use crate::util::Id;

pub fn transitive_closure<T: Clone + Eq + PartialOrd + Ord>(
    mut map: BTreeMap<T, BTreeSet<T>>,
) -> BTreeMap<T, BTreeSet<T>> {
    let empty = BTreeSet::new();
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

pub fn symmetric_closure<T: Clone + Eq + PartialOrd + Ord>(
    map: &BTreeMap<T, BTreeSet<T>>,
) -> BTreeMap<T, BTreeSet<T>> {
    let mut clo = map.clone();
    for (node, succs) in map {
        for succ in succs {
            clo.get_mut(succ).unwrap().insert(node.clone());
        }
    }
    clo
}

pub fn inverse<T: Clone + Eq + PartialOrd + Ord>(
    map: &BTreeMap<T, BTreeSet<T>>,
) -> BTreeMap<T, BTreeSet<T>> {
    let mut inv: BTreeMap<_, BTreeSet<_>> = BTreeMap::new();
    for node in map.keys() {
        inv.insert(node.clone(), BTreeSet::new());
    }
    for (node, succs) in map {
        for succ in succs {
            inv.get_mut(succ).unwrap().insert(node.clone());
        }
    }
    inv
}

/// `map` must not have a cycle.
pub fn post_order<T: Clone + Eq + PartialOrd + Ord>(
    map: &BTreeMap<T, BTreeSet<T>>,
    inv_map: &BTreeMap<T, BTreeSet<T>>,
) -> Vec<Vec<T>> {
    let mut res = vec![];
    let clo = symmetric_closure(map);
    let (_, components) = compute_sccs(&clo);

    for (_, component) in components {
        let mut v = vec![];
        let mut reached = BTreeSet::new();
        for node in component {
            if inv_map.get(&node).unwrap().is_empty() {
                dfs_walk(&node, &mut v, &mut reached, map);
            }
        }
        res.push(v);
    }

    res
}

fn dfs_walk<T: Clone + Eq + PartialOrd + Ord>(
    node: &T,
    v: &mut Vec<T>,
    reached: &mut BTreeSet<T>,
    map: &BTreeMap<T, BTreeSet<T>>,
) {
    reached.insert(node.clone());
    for succ in map.get(node).unwrap() {
        if !reached.contains(succ) {
            dfs_walk(succ, v, reached, map);
        }
    }
    v.push(node.clone());
}

/// `map` must not have a cycle.
pub fn reverse_post_order<T: Clone + Eq + PartialOrd + Ord>(
    map: &BTreeMap<T, BTreeSet<T>>,
    inv_map: &BTreeMap<T, BTreeSet<T>>,
) -> Vec<Vec<T>> {
    let mut po = post_order(map, inv_map);
    for v in &mut po {
        v.reverse();
    }
    po
}

pub fn compute_sccs<T: Clone + Eq + PartialOrd + Ord>(
    map: &BTreeMap<T, BTreeSet<T>>,
) -> (BTreeMap<Id, BTreeSet<Id>>, BTreeMap<Id, BTreeSet<T>>) {
    let id_map: BTreeMap<_, _> = map
        .keys()
        .enumerate()
        .map(|(i, f)| (i, f.clone()))
        .collect();
    let inv_id_map: BTreeMap<_, _> = id_map.iter().map(|(i, f)| (f.clone(), *i)).collect();
    let edges = map
        .iter()
        .flat_map(|(node, succs)| {
            succs.iter().map(|succ| {
                (
                    Id::new(*inv_id_map.get(node).unwrap()),
                    Id::new(*inv_id_map.get(succ).unwrap()),
                )
            })
        })
        .collect();
    let sccs: Sccs<Id, Id> = Sccs::new(&VecGraph::new(map.len(), edges));

    let component_graph: BTreeMap<_, _> = sccs
        .all_sccs()
        .map(|node| (node, sccs.successors(node).iter().cloned().collect()))
        .collect();

    let mut component_elems: BTreeMap<_, BTreeSet<_>> = BTreeMap::new();
    for i in 0..(map.len()) {
        let scc = sccs.scc(Id::new(i));
        let node = id_map.get(&i).unwrap().clone();
        component_elems.entry(scc).or_default().insert(node);
    }

    (component_graph, component_elems)
}

#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, BTreeSet};

    #[test]
    fn test1() {
        let mut graph = BTreeMap::new();
        graph.insert(1, BTreeSet::from([2]));
        graph.insert(2, BTreeSet::from([3]));
        graph.insert(3, BTreeSet::from([4, 5]));
        graph.insert(4, BTreeSet::from([6]));
        graph.insert(5, BTreeSet::from([6]));
        graph.insert(6, BTreeSet::from([]));

        let closure = super::transitive_closure(graph.clone());
        assert_eq!(closure.get(&1).unwrap(), &BTreeSet::from([2, 3, 4, 5, 6]));
        assert_eq!(closure.get(&2).unwrap(), &BTreeSet::from([3, 4, 5, 6]));
        assert_eq!(closure.get(&3).unwrap(), &BTreeSet::from([4, 5, 6]));
        assert_eq!(closure.get(&4).unwrap(), &BTreeSet::from([6]));
        assert_eq!(closure.get(&5).unwrap(), &BTreeSet::from([6]));
        assert_eq!(closure.get(&6).unwrap(), &BTreeSet::from([]));

        let inv_graph = super::inverse(&graph);
        assert_eq!(inv_graph.get(&1).unwrap(), &BTreeSet::from([]));
        assert_eq!(inv_graph.get(&2).unwrap(), &BTreeSet::from([1]));
        assert_eq!(inv_graph.get(&3).unwrap(), &BTreeSet::from([2]));
        assert_eq!(inv_graph.get(&4).unwrap(), &BTreeSet::from([3]));
        assert_eq!(inv_graph.get(&5).unwrap(), &BTreeSet::from([3]));
        assert_eq!(inv_graph.get(&6).unwrap(), &BTreeSet::from([4, 5]));

        let mut rpo = super::reverse_post_order(&graph, &inv_graph);
        assert_eq!(rpo.len(), 1);
        let v = rpo.pop().unwrap();
        assert_eq!(&v[0..3], &vec![1, 2, 3]);
        assert_eq!(
            v[3..5].iter().cloned().collect::<BTreeSet<_>>(),
            BTreeSet::from([4, 5])
        );
        assert_eq!(&v[5], &6);
    }

    #[test]
    fn test2() {
        let mut graph = BTreeMap::new();
        graph.insert(1, BTreeSet::from([1, 2]));
        graph.insert(2, BTreeSet::from([2, 1]));

        let closure = super::transitive_closure(graph.clone());
        assert_eq!(graph, closure);

        let inv_graph = super::inverse(&graph);
        assert_eq!(graph, inv_graph);
    }

    #[test]
    fn test3() {
        let mut graph = BTreeMap::new();
        graph.insert(1, BTreeSet::from([]));
        graph.insert(2, BTreeSet::from([]));

        let closure = super::transitive_closure(graph.clone());
        assert_eq!(graph, closure);

        let inv_graph = super::inverse(&graph);
        assert_eq!(graph, inv_graph);

        let mut rpo = super::reverse_post_order(&graph, &inv_graph);
        assert_eq!(rpo.len(), 2);
        let mut v1 = rpo.pop().unwrap();
        let mut v2 = rpo.pop().unwrap();
        assert_eq!(v1.len(), 1);
        assert_eq!(v2.len(), 1);
        v1.append(&mut v2);
        assert_eq!(
            v1.drain(..).collect::<BTreeSet<_>>(),
            BTreeSet::from([1, 2])
        );
    }

    #[test]
    fn test4() {
        let mut graph = BTreeMap::new();
        graph.insert(1, BTreeSet::from([3]));
        graph.insert(2, BTreeSet::from([3]));
        graph.insert(3, BTreeSet::from([]));

        let closure = super::transitive_closure(graph.clone());
        assert_eq!(graph, closure);

        let inv_graph = super::inverse(&graph);
        assert_eq!(inv_graph.get(&1).unwrap(), &BTreeSet::from([]));
        assert_eq!(inv_graph.get(&2).unwrap(), &BTreeSet::from([]));
        assert_eq!(inv_graph.get(&3).unwrap(), &BTreeSet::from([1, 2]));

        let mut rpo = super::reverse_post_order(&graph, &inv_graph);
        assert_eq!(rpo.len(), 1);
        let v = rpo.pop().unwrap();
        assert!(v == vec![1, 2, 3] || v == vec![2, 1, 3]);
        assert_eq!(
            v[0..2].iter().cloned().collect::<BTreeSet<_>>(),
            BTreeSet::from([1, 2])
        );
        assert_eq!(&v[2], &3);
    }

    #[test]
    fn test5() {
        let mut graph = BTreeMap::new();
        graph.insert(1, BTreeSet::from([2, 3, 4]));
        graph.insert(2, BTreeSet::from([3, 4]));
        graph.insert(3, BTreeSet::from([4]));
        graph.insert(4, BTreeSet::from([]));

        let closure = super::transitive_closure(graph.clone());
        assert_eq!(graph, closure);

        let inv_graph = super::inverse(&graph);
        assert_eq!(inv_graph.get(&1).unwrap(), &BTreeSet::from([]));
        assert_eq!(inv_graph.get(&2).unwrap(), &BTreeSet::from([1]));
        assert_eq!(inv_graph.get(&3).unwrap(), &BTreeSet::from([1, 2]));
        assert_eq!(inv_graph.get(&4).unwrap(), &BTreeSet::from([1, 2, 3]));

        let mut rpo = super::reverse_post_order(&graph, &inv_graph);
        assert_eq!(rpo.len(), 1);
        let v = rpo.pop().unwrap();
        assert_eq!(v, vec![1, 2, 3, 4]);
    }
}
