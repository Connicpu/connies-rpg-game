use specs::{System, FetchMut, WriteStorage, Entity};
use wrapped2d;

use std::sync::Mutex;
use std::ops::{Deref, DerefMut};

pub struct World {
    pub lock: Mutex<WorldWrapper>,
}

pub mod body;
pub mod run;
pub mod update;

pub struct EntityUserData;

impl wrapped2d::user_data::UserDataTypes for EntityUserData {
    type BodyData = Entity;
    type JointData = ();
    type FixtureData = ();
}

pub struct WorldWrapper(pub wrapped2d::b2::World<EntityUserData>);
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
