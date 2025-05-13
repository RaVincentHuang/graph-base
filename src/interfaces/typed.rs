
pub trait Type {
    fn type_id(&self) -> usize;
    // fn type_name(&self) -> String;
}

pub trait Typed {
    fn type_same(&self, x: & impl Type, y: & impl Type) -> bool;
}
