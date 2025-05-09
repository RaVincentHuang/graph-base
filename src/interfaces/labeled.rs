use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;
use crate::interfaces::graph::Graph;

use crate::interfaces::graph::{Adjacency, SingleId, IdPair};


/// Label for `Node`
pub trait Label: Hash + Eq + Clone + Display {
    fn label(&self) -> &str;
}

pub trait Labeled<'a>: Graph<'a>  {
    fn label_same(&self, node: &Self::Node, label: &Self::Node) -> bool;
    fn get_label(&'a self, node: &'a Self::Node) -> &'a impl Label;
    fn get_edges_pair_label(&'a self) -> impl Iterator<Item = (&'a Self::Node, &'a Self::Node, &'a impl Label)>;
    fn edge_label_same(&self, edge1: &Self::Edge, edge2: &Self::Edge) -> bool;
    fn edge_node_label_same(&self, src1: &Self::Node, edge1: &Self::Edge, dst1: &Self::Node, src2: &Self::Node, edge2: &Self::Edge, dst2: &Self::Node) -> bool;
}

impl Label for String {
    fn label(&self) -> &str {
        self.as_str()
    }
}

pub trait HyperLabeled<'a>: Labeled<'a> {
    type L: Label;
    fn set_same_label_fn(&mut self, f: Box<dyn Fn(&Self::L, &Self::L) -> bool>);
}

pub struct LabeledAdjacencyList<'a, T: Graph<'a>>(HashMap<&'a T::Node, Vec<(&'a T::Node, &'a T::Edge)>>);

pub trait LabeledAdjacency<'a>: Adjacency<'a> + Labeled<'a> 
where <Self as Graph<'a>>::Edge: IdPair {
    fn get_labeled_adj(&'a self) -> LabeledAdjacencyList<'a, Self> {

        let mut id_map = HashMap::new();
        for node in self.nodes() {
            id_map.insert(node.id(), node);
        }

        let mut adj = HashMap::new();
        for node in self.nodes() {
            adj.insert(node, Vec::new());
        }
        
        for edge in self.edges() {
            let (src, dst) = (id_map.get(&edge.pair().0).unwrap(), id_map.get(&edge.pair().1).unwrap());
            adj.get_mut(src).unwrap().push((dst.clone(), edge));
        }

        LabeledAdjacencyList(adj)
    }
    fn get_labeled_post(&'a self, adj: &LabeledAdjacencyList<'a, Self>, node: &Self::Node) -> impl Iterator<Item = (&'a Self::Node, &'a Self::Edge)> {
        adj.0.get(node).expect(format!("No node in adjacency table named {}", node).as_str()).iter().copied()
    }
}
