use std::{fmt::Display, hash::Hash};
use crate::interfaces::graph::SingleId;

// pub trait Vertex: Eq + Hash + Clone + Sized + Display + SingleId {
    
// }

pub trait Vertex: Eq + Hash + Clone + Sized + Display + SingleId {}

// pub struct VertexWrapper<T: Vertex>(pub T);

// impl<T: Vertex> PartialEq for VertexWrapper<T> {
//     fn eq(&self, other: &Self) -> bool {
//         self.0.id() == other.0.id()
//     }
// }
