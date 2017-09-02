use wrapped2d::{self, b2};

use conniecs::Entity;

pub use self::body::Body;
pub use self::run::PhysicsRun;
pub use self::update::PhysicsUpdate;

pub mod ext;
pub mod body;
pub mod run;
pub mod update;

pub const GRAVITY: b2::Vec2 = b2::Vec2 { x: 0.0, y: -15.0 };

pub struct World {
    pub world: b2::World<EntityUserData>,
}

impl World {
    pub fn new() -> Self {
        let world = b2::World::new(&GRAVITY);
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
    type FixtureData = Entity;
}
