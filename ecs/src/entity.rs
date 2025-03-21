use std::any::type_name;

use crate::world::World;

pub struct EntityBuilder<'a>
{
    id: usize,
    world: &'a mut World,
}

impl<'a> EntityBuilder<'a>
{
    pub fn new(id: usize, world: &'a mut World) -> Self
    {
        Self { id, world }
    }

    pub fn with<T: 'static>(self, component: T) -> Self
    {
        if let Some(storage) = self.world.get_storage_mut::<T>()
        {
            storage.add(self.id, component);
            return self;
        }
        panic!(
            "You cannot spawn an entity `{}` with unregistered type `{}`.",
            self.id,
            type_name::<T>()
        );
    }

    #[inline]
    pub fn into_id(self) -> usize
    {
        self.id
    }
}
