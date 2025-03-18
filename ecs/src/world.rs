use std::any::TypeId;
use std::collections::HashMap;

use crate::components::{ComponentStorage, SparseSet};

pub struct World
{
    size: usize,
    next_free_entity_id: usize,
    components: HashMap<TypeId, Box<dyn ComponentStorage>>,
}

impl World
{
    pub fn new(size: usize) -> Self
    {
        Self {
            size,
            next_free_entity_id: 0,
            components: HashMap::new(),
        }
    }

    pub fn spawn(&mut self) -> usize
    {
        // FIXME: This is `VERY` temporary.
        // The whole entity creation system should be much better.
        // A nice abstraction allowing to do this in a single pass would
        // be very nice.
        let entity_id = self.next_free_entity_id;
        self.next_free_entity_id += 1;

        entity_id
    }

    pub fn create<T: 'static>(&mut self, id: usize, item: T)
    {
        self.get_storage_mut().map(|storage| storage.add(id, item));
    }

    pub fn get<T: 'static>(&self, id: usize) -> Option<&T>
    {
        self.get_storage().and_then(|storage| storage.get(id))
    }

    pub fn get_mut<T: 'static>(&mut self, id: usize) -> Option<&mut T>
    {
        self.get_storage_mut()
            .and_then(|storage| storage.get_mut(id))
    }

    pub fn register<T: 'static>(mut self) -> Self
    {
        let type_id = TypeId::of::<T>();

        self.components
            .entry(type_id)
            .or_insert_with(|| Box::new(SparseSet::<T>::new(self.size)));

        self
    }

    fn get_storage<T: 'static>(&self) -> Option<&SparseSet<T>>
    {
        let type_id = TypeId::of::<T>();

        self.components
            .get(&type_id)
            .and_then(|components| components.as_any().downcast_ref::<SparseSet<T>>())
    }

    fn get_storage_mut<T: 'static>(&mut self) -> Option<&mut SparseSet<T>>
    {
        let type_id = TypeId::of::<T>();

        self.components
            .get_mut(&type_id)
            .and_then(|components| components.as_any_mut().downcast_mut::<SparseSet<T>>())
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn register_entity_adds_entry_to_components()
    {
        let world = World::new(10).register::<u32>();
        let key = TypeId::of::<u32>();

        assert_eq!(world.components.len(), 1);
        assert_eq!(world.components.contains_key(&key), true);
    }

    #[test]
    fn full_entity_creation()
    {
        let mut world = World::new(10).register::<u32>().register::<&str>();
        let entity = world.spawn();

        world.create::<u32>(entity, 25);
        world.create::<&str>(entity, "name");

        let age = world.get::<u32>(entity);
        let name = world.get::<&str>(entity);

        assert_eq!(entity, 0);
        assert_eq!(age, Some(&25));
        assert_eq!(name, Some(&"name"));
    }
}
