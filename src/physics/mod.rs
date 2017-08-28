use wrapped2d::{self, b2};

use conniecs::Entity;

pub use self::body::Body;
pub use self::run::PhysicsRun;
pub use self::update::PhysicsUpdate;

pub mod body;
pub mod run;
pub mod update;

pub struct World {
    pub world: b2::World<EntityUserData>,
}

impl World {
    pub fn new() -> Self {

        let world = b2::World::<EntityUserData>::new(&b2::Vec2 { x: 0.0, y: -9.8 });

        World { world }
    }
}

impl Default for World {
    fn default() -> Self {
        World::new()
    }
}

pub struct EntityUserData;

impl wrapped2d::user_data::UserDataTypes for EntityUserData {
    type BodyData = Entity;
    type JointData = ();
    type FixtureData = ();
}
