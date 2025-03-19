use std::any::TypeId;
use std::collections::HashMap;

use crate::components::{ComponentStorage, SparseSet};
use crate::entity::EntityBuilder;

pub struct World
{
    size: usize,
    current_id: usize,
    components: HashMap<TypeId, Box<dyn ComponentStorage>>,
}

impl World
{
    pub fn new(size: usize) -> Self
    {
        Self {
            size,
            current_id: 0,
            components: HashMap::new(),
        }
    }

    pub fn spawn(&mut self) -> EntityBuilder<'_>
    {
        let spawned_entity_id = self.current_id;
        self.current_id += 1;

        EntityBuilder::new(spawned_entity_id, self)
    }

    pub fn despawn(&mut self, id: usize)
    {
        self.components
            .values_mut()
            .for_each(|components| components.delete(id))
    }

    pub fn get<T: 'static>(&self, id: usize) -> Option<&T>
    {
        self.get_storage::<T>().and_then(|storage| storage.get(id))
    }

    pub fn get_mut<T: 'static>(&mut self, id: usize) -> Option<&mut T>
    {
        self.get_storage_mut::<T>()
            .and_then(|storage| storage.get_mut(id))
    }

    pub(crate) fn get_storage<T: 'static>(&self) -> Option<&SparseSet<T>>
    {
        let type_id = TypeId::of::<T>();

        self.components
            .get(&type_id)
            .and_then(|components| components.as_any().downcast_ref::<SparseSet<T>>())
    }

    pub(crate) fn get_storage_mut<T: 'static>(&mut self) -> Option<&mut SparseSet<T>>
    {
        let type_id = TypeId::of::<T>();

        self.components
            .get_mut(&type_id)
            .and_then(|components| components.as_any_mut().downcast_mut::<SparseSet<T>>())
    }

    pub fn register<T: 'static>(mut self) -> Self
    {
        let type_id = TypeId::of::<T>();

        self.components
            .entry(type_id)
            .or_insert_with(|| Box::new(SparseSet::<T>::new(self.size)));
        self
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    #[should_panic]
    fn spawn_entity_with_unregistered_component()
    {
        let mut world = World::new(10);
        world.spawn().with::<u32>(25).into_id();
    }

    #[test]
    fn spawn_entity()
    {
        let mut world = World::new(10).register::<u32>().register::<String>();

        let entity_id = world
            .spawn()
            .with::<u32>(25)
            .with::<String>("test".to_string())
            .into_id();

        let age = world.get::<u32>(entity_id);
        let name = world.get::<String>(entity_id);

        assert_eq!(age, Some(&25));
        assert_eq!(name, Some(&"test".to_string()));
    }

    #[test]
    fn despawn_entity()
    {
        let mut world = World::new(10).register::<u32>().register::<i32>();
        let entity_id = world.spawn().with::<u32>(25).with::<i32>(-10).into_id();

        world.despawn(entity_id);

        let v1 = world.get::<u32>(entity_id);
        let v2 = world.get::<i32>(entity_id);

        assert!(v1.is_none());
        assert!(v2.is_none());
    }

    #[test]
    fn get_entity_component_doesnt_exist()
    {
        let mut world = World::new(10).register::<u32>().register::<i32>();
        let entity_id = world.spawn().with::<u32>(25).into_id();

        let value = world.get::<i32>(entity_id);

        assert!(value.is_none());
    }

    #[test]
    fn get_entity_component_when_not_registered()
    {
        let world = World::new(10);
        let value = world.get::<u32>(0);

        assert!(value.is_none());
    }

    #[test]
    fn get_entity_component()
    {
        let mut world = World::new(10).register::<u32>();
        let entity_id = world.spawn().with::<u32>(25).into_id();

        let value = world.get::<u32>(entity_id);

        assert_eq!(value, Some(&25));
    }

    #[test]
    fn get_mut_entity_component_doesnt_exist()
    {
        let mut world = World::new(10).register::<u32>().register::<i32>();
        let entity_id = world.spawn().with::<u32>(25).into_id();

        let value = world.get_mut::<i32>(entity_id);

        assert!(value.is_none());
    }

    #[test]
    fn get_mut_entity_component_when_not_registered()
    {
        let mut world = World::new(10);
        let value = world.get_mut::<u32>(0);

        assert!(value.is_none());
    }

    #[test]
    fn get_mut_entity_component()
    {
        let mut world = World::new(10).register::<u32>();
        let entity_id = world.spawn().with::<u32>(25).into_id();

        let value = world.get_mut::<u32>(entity_id);

        assert_eq!(value, Some(&mut 25));

        *value.unwrap() = 36;

        let value = world.get::<u32>(entity_id);

        assert_eq!(value, Some(&36));
    }

    #[test]
    fn register_entity_adds_entry_to_components()
    {
        let world = World::new(10).register::<u32>();
        let key = TypeId::of::<u32>();

        assert_eq!(world.components.len(), 1);
        assert_eq!(world.components.contains_key(&key), true);
    }
}
