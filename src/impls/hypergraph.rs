use std::fmt::Display;
use std::sync::RwLock;

use crate::interfaces::edge::{DirectedHyperedge, Hyperedge, NodeSet};
use crate::interfaces::graph::SingleId;
use crate::interfaces::hypergraph::{Hypergraph, IdVector};
use crate::interfaces::typed::Type;
use crate::interfaces::vertex::Vertex;

use std::{collections::{HashMap, HashSet}, hash::Hash, ops::{Add, BitXor, Div, Mul, Sub}};
use rand::{prelude::*, rng};
use rand::distr::StandardUniform;
use serde::{Serialize, Deserialize};
use lazy_static::lazy_static;

lazy_static!{
    static ref clusters: RwLock<HashMap<usize, Desc>> = RwLock::new(HashMap::new());
}
#[derive(Clone, Serialize, Deserialize)]
pub struct Desc([f64; 16]);

impl Hash for Desc {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for value in &self.0 {
            let bits = value.to_bits();
            bits.hash(state);
        }
    }
}

impl PartialEq for Desc {
    fn eq(&self, other: &Self) -> bool {
        self.0.iter().zip(other.0.iter()).all(|(x, y)| x == y)
    }
}

impl Eq for Desc {}

impl Add for Desc {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let mut result = [0.0; 16];
        for i in 0..16 {
            result[i] = self.0[i] + other.0[i];
        }
        Desc(result)
    }
}

impl Sub for Desc {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        let mut result = [0.0; 16];
        for i in 0..16 {
            result[i] = self.0[i] - other.0[i];
        }
        Desc(result)
    }
}

impl Mul for Desc {
    type Output = f64;
    fn mul(self, other: Self) -> f64 {
        let mut result = 0.0;
        for i in 0..16 {
            result += self.0[i] * other.0[i];
        }
        result
    }
}

impl Mul<f64> for Desc {
    type Output = Desc;
    fn mul(self, scalar: f64) -> Desc {
        let mut result = [0.0; 16];
        for i in 0..16 {
            result[i] = self.0[i] * scalar;
        }
        Desc(result)
    }
}

impl Div<f64> for Desc {
    type Output = Desc;
    fn div(self, scalar: f64) -> Desc {
        let mut result = [0.0; 16];
        for i in 0..16 {
            result[i] = self.0[i] / scalar;
        }
        Desc(result)
    }
}

// cosine for Desc
impl BitXor for Desc {
    type Output = f64;
    fn bitxor(self, other: Self) -> f64 {
        let res = self.clone() * other.clone();
        let norm1 = self.clone() * self.clone();
        let norm2 = other.clone() * other.clone();
        res / (norm1.sqrt() * norm2.sqrt())
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
pub struct NodeType(usize);

impl Type for NodeType {
    fn type_id(&self) -> usize {
        self.0
    }
}

impl Display for NodeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl NodeType {
    pub fn new(id: usize) -> Self {
        NodeType(id)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
pub struct Node {
    id: usize,
    node_type: NodeType,
    desc: Desc
}

impl BitXor for Node {
    type Output = f64;
    fn bitxor(self, other: Self) -> f64 {
        return self.desc.clone() ^ other.desc.clone();
    }
}

fn generate_orthogonal_unit(base: &Desc) -> Desc {
    let base_norm = (base.clone() * base.clone()).sqrt();
    let mut orthogonal = Desc([0.0; 16]);
    
    let mut rng = rng();
    loop {
        // 生成随机高斯向量
        for i in 0..16 {
            orthogonal.0[i] = rng.sample(StandardUniform);
        }
        
        
        // 计算与基向量的点积
        let projection = (orthogonal.clone() * base.clone()) / base_norm;
        
        // 减去投影分量使其正交
        // for i in 0..16 {
        //     orthogonal[i] -= projection * base[i] / base_norm;
        // }

        orthogonal = orthogonal - (base.clone() * projection) / base_norm;
        
        // 归一化处理
        let ortho_norm = (orthogonal.clone() * orthogonal.clone()).sqrt();
        if ortho_norm > 1e-10 {
            orthogonal = orthogonal / ortho_norm;
            break;
        }
    }
    orthogonal
}


impl Node {
    pub fn from_random(id: usize, k: usize, p: f64, alpha: f64, rng: &mut impl Rng) -> Node {
        // get A random [f64; 16]
        let random_type = rng.random_range(0..k);
        let desc = {
            let random_vec: [f64; 16] = rng.sample(StandardUniform);
            let desc = Desc(random_vec);
            
            if !clusters.read().unwrap().contains_key(&random_type) {
                if clusters.read().unwrap().is_empty() {
                    clusters.write().unwrap().insert(random_type, desc.clone());
                    desc
                } else {
                    let avg_vec = clusters.read().unwrap().iter().map(|(_, v)| v.clone()).reduce(|a, b| a + b).unwrap();
                    let orthogonal = generate_orthogonal_unit(&avg_vec);
                    let res = orthogonal + desc;
                    clusters.write().unwrap().insert(random_type, res.clone());
                    res
                }
            } else {
                let cluster_guard = clusters.read().unwrap();
                let cluster_desc = cluster_guard.get(&random_type).unwrap().clone();
    
                if rng.random_bool(p) {
                    let res = cluster_desc.clone() * (1.0 - alpha) + desc * alpha;
                    res
                } else {
                    let orthogonal = generate_orthogonal_unit(&cluster_desc);
                    let res = orthogonal * (1.0 - alpha) + desc * alpha;
                    res
                }
            }
        };

        Node {
            id,
            node_type: NodeType::new(random_type),
            desc
        }
    }
}

impl SingleId for Node {
    fn id(&self) -> usize {
        self.id
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Node {{ id: {}, node_type: {} }}", self.id, self.node_type)
    }
}

impl Vertex for Node {}


#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
pub struct HyperedgeImpl {
    nodes: Vec<usize>,
}

impl IdVector for HyperedgeImpl {
    fn id(&self) -> Vec<usize> {
        self.nodes.clone()
    }
}

impl NodeSet for HyperedgeImpl {
    fn from_nodes(nodes: Vec<usize>) -> Self {
        HyperedgeImpl { nodes }
    }
}

impl Hyperedge for HyperedgeImpl {
    fn id_set(&self) -> HashSet<usize> {
        self.nodes.iter().cloned().collect()
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
pub struct DirectedHyperedgeImpl {
    src: Vec<usize>,
    dst: Vec<usize>,
}

impl IdVector for DirectedHyperedgeImpl {
    fn id(&self) -> Vec<usize> {
        self.src.iter().chain(self.dst.iter()).cloned().collect()
    }
}

impl DirectedHyperedge for DirectedHyperedgeImpl {
    fn src(&self) -> HashSet<usize> {
        self.src.iter().cloned().collect()
    }

    fn dst(&self) -> HashSet<usize> {
        self.dst.iter().cloned().collect()
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
pub struct HypergraphImpl {
    nodes: Vec<Node>,
    edges: Vec<HyperedgeImpl>,
}

impl<'a> Hypergraph<'a> for HypergraphImpl {
    type Node = Node;
    type Edge = HyperedgeImpl;

    fn new() -> Self {
        HypergraphImpl {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    fn nodes(&'a self) -> impl Iterator<Item = &'a Self::Node> {
        self.nodes.iter()
    }

    fn hyperedges(&'a self) -> impl Iterator<Item = &'a Self::Edge> {
        self.edges.iter()
    }

    fn add_node(&mut self, node: Self::Node) {
        self.nodes.push(node);
    }

    fn add_hyperedge(&mut self, edge: Self::Edge) {
        self.edges.push(edge);
    }
}


#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
pub struct DirectedHypergraphImpl {
    nodes: Vec<Node>,
    edges: Vec<DirectedHyperedgeImpl>,
}

impl DirectedHypergraphImpl {
    pub fn new() -> Self {
        DirectedHypergraphImpl {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.push(node);
    }

    pub fn add_hyperedge(&mut self, edge: DirectedHyperedgeImpl) {
        self.edges.push(edge);
    }
}
