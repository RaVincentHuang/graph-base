use std::hash::Hash;
use std::collections::HashSet;

pub trait Hyperedge: Eq + Hash + Clone {
    fn id_set(&self) -> HashSet<usize>;
    fn is_subset(&self, other: &Self) -> bool {
        self.id_set().is_subset(&other.id_set())
    }
    fn is_equal(&self, other: &Self) -> bool {
        self.id_set().is_subset(&other.id_set()) && other.id_set().is_subset(&self.id_set())
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

