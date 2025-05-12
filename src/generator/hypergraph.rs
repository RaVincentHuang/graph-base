
use crate::{generator::{RandomExpand, RandomGenerate, RandomModify, RandomShrink}, interfaces::{edge::{DirectedHyperedge, NodeSet, NodeSetPair, Hyperedge}, graph::{Directed, SingleId, UnDirected}, hypergraph::{Hypergraph}, vertex::Vertex}};

// impl<V> RandomGenerate for V 
// where V: Vertex {
//     fn random_generate(n: usize, _e: usize, rng: &mut impl rand::Rng) -> Self {
//         let id = rng.gen_range(0..n);
//         Self::new(id)
//     }
// }

pub trait NodeSample<'a>: Hypergraph<'a> {
    fn sample(&self, rng: &mut impl rand::Rng) -> Vec<&Self::Node>;
}

impl<'a, H> RandomGenerate for H 
where H: NodeSample<'a>, H::Node: RandomGenerate + SingleId, H::Edge: NodeSet {
    fn random_generate(n: usize, e: usize, rng: &mut impl rand::Rng) -> Self {
        let mut hypergraph = Self::new();
        for i in 0..n {
            let new_node = H::Node::random_generate(i, e, rng);
            hypergraph.add_node(new_node);
        }
        for _ in 0..e {
            let nodes = hypergraph.sample(rng);
            let nodes = nodes.iter().map(|node| node.id()).collect::<Vec<_>>();
            let new_edge = H::Edge::from_nodes(nodes);
            hypergraph.add_hyperedge(new_edge);
        }
        hypergraph
    }
}

impl<'a, H> RandomExpand for H 
where H: NodeSample<'a> + Clone, H::Node: RandomGenerate + SingleId, H::Edge: NodeSet {
    fn random_expand(&self, n_plus: usize, e_plus: usize, rng: &mut impl rand::Rng) -> Self {
        let mut hypergraph = self.clone();
        for _ in 0..n_plus {
            let new_node = H::Node::random_generate(n_plus, e_plus, rng);
            hypergraph.add_node(new_node);
        }
        for _ in 0..e_plus {
            let nodes = hypergraph.sample(rng);
            let nodes = nodes.iter().map(|node| node.id()).collect::<Vec<_>>();
            let new_edge = H::Edge::from_nodes(nodes);
            hypergraph.add_hyperedge(new_edge);
        }
        hypergraph
    }
}

