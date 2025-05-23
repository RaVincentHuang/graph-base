use std::collections::HashMap;
use std::fmt::Display;

use crate::interfaces::labeled::{Label, Labeled, HyperLabeled, LabeledAdjacency};
use crate::interfaces::graph::{SingleId, IdPair, Graph, Adjacency, AdjacencyInv, Directed};
use crate::interfaces::vertex::Vertex;

#[derive(Hash, Eq, PartialEq, Clone)]
pub struct LabelNode<L: Label> {
    id: u64,
    label: L
}

impl<L: Label> Display for LabelNode<L> 
where L: Label {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[id: {}, label: {}]", self.id, self.label)
    }
}

impl<L: Label> SingleId for LabelNode<L> {
    fn id(&self) -> usize {
        self.id as usize
    }
}

impl<L: Label> Label for LabelNode<L> {
    fn label(&self) -> &str {
        self.label.label()
    }
}

#[derive(Hash, Eq, PartialEq, Clone)]
pub struct LabeledEdge<L: Label> {
    src: u64,
    dst: u64,
    label: L,
}

impl<L: Label> IdPair for LabeledEdge<L> {
    fn pair(&self) -> (usize, usize) {
        (self.src as usize, self.dst as usize)
    }
    
}

impl<L: Label> Display for LabeledEdge<L> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{} -> {}: label: {}]", self.src, self.dst, self.label)
    }
}

impl<L: Label> Label for LabeledEdge<L> {
    fn label(&self) -> &str {
        self.label.label()
    }
}

/// `L1` is the label of the nodes, `L2` is the label of the edges. All these labels needs to implement the `Label` trait.
/// 
/// The graph is directed.

// pub trait LabeledGraph<'a>: Graph<'a> 
//     where <Self as Graph<'a>>::Node: Label, <Self as Graph<'a>>::Edge: Label {}
pub struct SimpleLabeledGraph<L1: Label, L2: Label> {
    nodes: Vec<LabelNode<L1>>,
    edges: Vec<LabeledEdge<L2>>,
}

#[derive(Hash, Eq, Clone)]
pub struct SingleLabel(());

impl Display for SingleLabel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

impl Label for SingleLabel {
    fn label(&self) -> &str {
        ""
    }
}

impl PartialEq for SingleLabel {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

pub type StandardLabeledGraph = SimpleLabeledGraph<String, SingleLabel>;

impl<'a> Graph<'a> for StandardLabeledGraph {
    type Node = LabelNode<String>;

    type Edge = LabeledEdge<SingleLabel>;

    fn new() -> Self {
        StandardLabeledGraph {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    fn nodes(&'a self) -> impl Iterator<Item = &'a Self::Node> {
        self.nodes.iter()
    }

    fn edges(&'a self) -> impl Iterator<Item = &'a Self::Edge> {
        self.edges.iter()
    }
    

    fn add_node(&mut self, node: Self::Node) {
        self.nodes.push(node);
    }

    fn add_edge(&mut self, edge: Self::Edge) {
        self.edges.push(edge);
    }
}

impl<'a> Labeled<'a> for StandardLabeledGraph {
    fn label_same(&self, node: &Self::Node, label: &Self::Node) -> bool {
        node.label == label.label
    }

    fn get_label(&'a self, node: &'a Self::Node) -> &'a impl Label {
        &node.label
    }

    fn get_edges_pair_label(&'a self) -> impl Iterator<Item = (&'a Self::Node, &'a Self::Node, &'a impl Label)> {
        let id_map: HashMap<_, _, std::collections::hash_map::RandomState> = HashMap::from_iter(self.nodes.iter().map(|node| (node.id, node)));
        self.edges.iter().map(move |edge| (id_map.get(&edge.src).unwrap().clone(), id_map.get(&edge.dst).unwrap().clone(), &edge.label)).collect::<Vec<_>>().into_iter()
    }

    fn edge_label_same(&self, _: &Self::Edge, _: &Self::Edge) -> bool {
        true
    }

    fn edge_node_label_same(&self, _: &Self::Node, _: &Self::Edge, _: &Self::Node, _: &Self::Node, _: &Self::Edge, _: &Self::Node) -> bool {
        true
    }
}

impl Directed for StandardLabeledGraph {}

impl Adjacency<'_> for StandardLabeledGraph {}
    
impl AdjacencyInv<'_> for StandardLabeledGraph {}

impl LabeledAdjacency<'_> for StandardLabeledGraph {}

impl StandardLabeledGraph {
    pub fn new() -> Self {
        return StandardLabeledGraph {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn add_node(&mut self, id: u64, label: String) {
        self.nodes.push(LabelNode {
            id,
            label
        });
    }

    pub fn add_edge(&mut self, src: u64, dst: u64) {
        self.edges.push(LabeledEdge {
            src,
            dst,
            label: SingleLabel(())
        });
    }
}

impl Display for StandardLabeledGraph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        s.push_str("nodes: ");
        for node in self.nodes.iter() {
            s.push_str(format!("{}, ", node).as_str());
        }
        s.push_str("\nedges: ");
        for edge in self.edges.iter() {
            s.push_str(format!("{} -> {}, ", edge.src, edge.dst).as_str());
        }
        write!(f, "{}", s)
    }
}
impl<L: Label> Vertex for LabelNode<L> {}

pub struct HyperLabelGraph<L: Label> {
    nodes: Vec<LabelNode<L>>,
    edges: Vec<LabeledEdge<SingleLabel>>,
    same_label_fn: Option<Box<dyn Fn(&L, &L) -> bool>>,
}

impl<'a, L: Label> Graph<'a> for HyperLabelGraph<L> {
    type Node = LabelNode<L>;
    type Edge = LabeledEdge<SingleLabel>;

    fn new() -> Self {
        HyperLabelGraph {
            nodes: Vec::new(),
            edges: Vec::new(),
            same_label_fn: None,
        }
    }

    fn nodes(&'a self) -> impl Iterator<Item = &'a Self::Node> {
        self.nodes.iter()
    }

    fn edges(&'a self) -> impl Iterator<Item = &'a Self::Edge> {
        self.edges.iter()
    }

    fn add_node(&mut self, node: Self::Node) {
        self.nodes.push(node);
    }

    fn add_edge(&mut self, edge: Self::Edge) {
        self.edges.push(edge);
    }
}

impl<'a, L: Label> Labeled<'a> for HyperLabelGraph<L> {
    fn label_same(&self, node: &Self::Node, label: &Self::Node) -> bool {
        (self.same_label_fn.as_ref().expect("hyper compare function is not set"))(&node.label, &label.label)
    }

    fn get_label(&'a self, node: &'a Self::Node) -> &'a impl Label {
        &node.label
    }

    fn get_edges_pair_label(&'a self) -> impl Iterator<Item = (&'a Self::Node, &'a Self::Node, &'a impl Label)> {
        let id_map: HashMap<_, _, std::collections::hash_map::RandomState> = HashMap::from_iter(self.nodes.iter().map(|node| (node.id, node)));
        self.edges.iter().map(move |edge| (id_map.get(&edge.src).unwrap().clone(), id_map.get(&edge.dst).unwrap().clone(), &edge.label)).collect::<Vec<_>>().into_iter()
    }

    fn edge_label_same(&self, _: &Self::Edge, _: &Self::Edge) -> bool {
        true
    }

    fn edge_node_label_same(&self, _: &Self::Node, _: &Self::Edge, _: &Self::Node, _: &Self::Node, _: &Self::Edge, _: &Self::Node) -> bool {
        true
    }
}

impl<L: Label> HyperLabeled<'_> for HyperLabelGraph<L> {
    type L = L;
    fn set_same_label_fn(&mut self, f: Box<dyn Fn(&Self::L, &Self::L) -> bool>) {
        self.same_label_fn = Some(f);
    }
}

impl Directed for HyperLabelGraph<SingleLabel> {}

impl Adjacency<'_> for HyperLabelGraph<SingleLabel> {}
    
impl AdjacencyInv<'_> for HyperLabelGraph<SingleLabel> {}

impl<L: Label> HyperLabelGraph<L> {
    pub fn new() -> Self {
        return HyperLabelGraph {
            nodes: Vec::new(),
            edges: Vec::new(),
            same_label_fn: None,
        }
    }
    
}
