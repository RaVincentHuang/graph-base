pub mod hypergraph;

use rand::prelude::*;

pub trait RandomGenerate {
    fn random_generate(n: usize, e: usize, rng: &impl Rng) -> Self;

}

pub trait RandomExpand {
    fn random_expand(&self, n_plus: usize, e_plus: usize, rng: &impl Rng) -> Self;
}

pub trait RandomShrink {
    fn random_shrink(&self, n_minus: usize, e_minus: usize, rng: &impl Rng) -> Self;
}

pub trait RandomModify {
    fn random_modify(&self, n_modify: usize, e_modify: usize, rng: &impl Rng) -> Self;
}
