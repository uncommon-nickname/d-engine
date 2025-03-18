mod sparse_set;

use std::any::Any;

pub trait ComponentStorage
{
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

#[derive(Debug)]
struct Entry<T>
{
    id: usize,
    item: T,
}

#[derive(Debug)]
pub struct SparseSet<T>
{
    dense: Vec<Entry<T>>,
    sparse: Vec<Option<usize>>,
}
