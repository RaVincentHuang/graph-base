use std::hash::Hash;
use std::collections::HashSet;

use super::hypergraph::IdVector;

pub trait Hyperedge: Eq + Hash + Clone + IdVector {
    fn id_set(&self) -> HashSet<usize>;
    fn is_subset(&self, other: &Self) -> bool {
        self.id_set().is_subset(&other.id_set())
    }
    fn is_equal(&self, other: &Self) -> bool {
        self.id_set().is_subset(&other.id_set()) && other.id_set().is_subset(&self.id_set())
    }
    fn has_intersection(&self, other: &Self) -> bool {
        !self.is_disjoint(other)
    }
    fn is_disjoint(&self, other: &Self) -> bool {
        self.id_set().is_disjoint(&other.id_set())
    }
    fn is_empty(&self) -> bool {
        self.id_set().is_empty()
    }
    fn is_singleton(&self) -> bool {
        self.id_set().len() == 1
    }
}

pub trait DirectedHyperedge: Hyperedge {
    fn src(&self) -> HashSet<usize>;
    fn dst(&self) -> HashSet<usize>;
    fn id_set_pair(&self) -> (HashSet<usize>, HashSet<usize>) {
        (self.src(), self.dst())
    }
}

impl<T> Hyperedge for T 
where T: DirectedHyperedge {
    fn id_set(&self) -> HashSet<usize> {
        self.src().union(&self.dst()).cloned().collect()
    }
}

// A hyperedge is only a set of nodes.
pub trait NodeSet: Hyperedge {
    type Node;
    fn from_nodes(nodes: Vec<usize>) -> Self;
}

// A hyperedge is only a pair of sets of nodes.
pub trait NodeSetPair: DirectedHyperedge {
    type Node;
    fn from_nodes_pair(src: Vec<usize>, dst: Vec<usize>) -> Self;
}

