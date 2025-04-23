use std::fmt::Display;
use std::hash::Hash;
use crate::interfaces::graph::Graph;


pub trait Label: Hash + Eq + Clone + Display {
    fn label(&self) -> &str;
}

pub trait Labeled<'a>: Graph<'a> {
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
