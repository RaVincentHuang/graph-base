use std::{collections::HashMap, fmt::Display};
use crate::interfaces::graph::SingleId;
use crate::interfaces::edge::Hyperedge;

use super::{edge::DirectedHyperedge, vertex::Vertex};

pub trait IdVector {
    fn id(&self) -> Vec<usize>;
}

pub trait Hypergraph<'a> {
    type Node: Vertex;
    type Edge: Hyperedge;
    fn new() -> Self;
    fn nodes(&'a self) -> impl Iterator<Item = &'a Self::Node>;
    fn hyperedges(&'a self) -> impl Iterator<Item = &'a Self::Edge>;
    fn get_hyperedges_vector(&'a self) -> impl Iterator<Item = Vec<&'a Self::Node>> {
        let id_map: HashMap<_, _> = HashMap::from_iter(self.nodes().map(|node| (node.id(), node)));
        self.hyperedges().map(move |edge| edge.id().iter().map(|id| id_map.get(id).unwrap()).cloned().collect::<Vec<_>>()).collect::<Vec<_>>().into_iter()
    }
    fn get_hyperedges_vector_with_edge(&'a self) -> impl Iterator<Item = (&'a Self::Edge, Vec<&'a Self::Node>)> {
        let id_map: HashMap<_, _> = HashMap::from_iter(self.nodes().map(|node| (node.id(), node)));
        self.hyperedges().map(move |edge| (edge, edge.id().iter().map(|id| id_map.get(id).unwrap()).cloned().collect::<Vec<_>>())).collect::<Vec<_>>().into_iter()
    }
    fn add_node(&mut self, node: Self::Node);
    fn add_hyperedge(&mut self, edge: Self::Edge);
}


pub struct AdjacencyList<'a, T: Hypergraph<'a>>(HashMap<&'a T::Node, Vec<&'a T::Node>>);

impl<'a, T> Display for AdjacencyList<'a, T> 
where T: Hypergraph<'a> {    
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for (node, adj) in self.0.iter() {
            
            let s1 = format!("{}", node);
            let mut s2 = String::new();
            for node in adj {
                s2.push_str(format!("{}, ", node).as_str());
            }

            s.push_str(format!("Node {} -> {{{}}}\n", s1, s2).as_str());
        }
        write!(f, "{}", s)
    }
}

pub trait DirectedHypergraph<'a>: Hypergraph<'a> + Sized 
where Self::Edge: DirectedHyperedge {}
