mod sparse_set;

use std::any::Any;

pub trait ComponentStorage
{
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn delete(&mut self, id: usize);
}

#[derive(Debug)]
pub struct Entry<T>
{
    pub id: usize,
    pub item: T,
}

#[derive(Debug)]
pub struct SparseSet<T>
{
    dense: Vec<Entry<T>>,
    sparse: Vec<Option<usize>>,
}
