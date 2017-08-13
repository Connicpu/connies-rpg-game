use specs::{System, FetchMut, WriteStorage, Entity};
use wrapped2d;

use std::sync::{Mutex, MutexGuard};
use std::ops::{Deref, DerefMut};

pub use self::body::Body;

pub mod body;
pub mod run;
pub mod update;

pub struct World {
    lock: Mutex<WorldWrapper>,
}

impl World {
    pub fn new() -> Self {
        use wrapped2d::b2;

        let world = b2::World::<EntityUserData>::new(&b2::Vec2 { x: 0.0, y: -9.8 });

        World {
            lock: Mutex::new(WorldWrapper(world)),
        }
    }

    pub fn access(&mut self) -> &mut wrapped2d::b2::World<EntityUserData> {
        &mut self.lock.get_mut().unwrap().0
    }

    pub fn lock(&self) -> MutexGuard<WorldWrapper> {
        self.lock.lock().unwrap()
    }
}

pub struct EntityUserData;

impl wrapped2d::user_data::UserDataTypes for EntityUserData {
    type BodyData = Entity;
    type JointData = ();
    type FixtureData = ();
}

pub struct WorldWrapper(wrapped2d::b2::World<EntityUserData>);
unsafe impl Send for WorldWrapper {}

impl Deref for WorldWrapper {
    type Target = wrapped2d::b2::World<EntityUserData>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for WorldWrapper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
