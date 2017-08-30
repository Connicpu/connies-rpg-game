use physics as p;

use conniecs::Entity;

use physics::EntityUserData;

pub mod update;
pub use player::update::PlayerUpdate;

use wrapped2d::b2;
use wrapped2d::dynamics;
use wrapped2d::collision;
use wrapped2d::user_data::UserData;

use std::sync::RwLock;
use std::sync::Arc;

pub struct PlayerGroundDetector {
    pub grounded: bool,
    pub contact_count: u32,
    pub player_ground_sensor_entity: Entity,
}

impl PlayerGroundDetector {
    pub fn new(player_ground_sensor_entity: Entity) -> Self {
        PlayerGroundDetector {
            grounded: false,
            contact_count: 0,
            player_ground_sensor_entity: player_ground_sensor_entity,
        }
    }
}

pub struct PlayerGroundDetectorCallbacks {
    detector: Arc<RwLock<PlayerGroundDetector>>,
}

impl PlayerGroundDetectorCallbacks {
    pub fn new(detector: &Arc<RwLock<PlayerGroundDetector>>) -> Self {
        PlayerGroundDetectorCallbacks {
            detector: detector.clone(),
        }
    }
}

impl dynamics::world::callbacks::ContactListener<EntityUserData> for PlayerGroundDetectorCallbacks {
    fn begin_contact(
        &mut self,
        contact: dynamics::world::callbacks::ContactAccess<EntityUserData>,
    ) {
        //println!("contact begin");
        let mut detector_write = self.detector.write().unwrap();
        if (*contact.fixture_a.user_data() == detector_write.player_ground_sensor_entity) ||
            (*contact.fixture_b.user_data() == detector_write.player_ground_sensor_entity)
        {
            detector_write.contact_count += 1;
            detector_write.grounded = true;
            //println!("contact_count: {}", detector_write.contact_count);
        }
    }
    fn end_contact(&mut self, contact: dynamics::world::callbacks::ContactAccess<EntityUserData>) {
        //println!("contact end");
        let mut detector_write = self.detector.write().unwrap();
        if (*contact.fixture_a.user_data() == detector_write.player_ground_sensor_entity) ||
            (*contact.fixture_b.user_data() == detector_write.player_ground_sensor_entity)
        {
            detector_write.contact_count -= 1;
            detector_write.grounded = detector_write.contact_count > 0;
            //println!("contact_count: {}", detector_write.contact_count);
        }
    }
    fn pre_solve(
        &mut self,
        _: dynamics::world::callbacks::ContactAccess<EntityUserData>,
        _: &collision::Manifold,
    ) {
    }
    fn post_solve(
        &mut self,
        _: dynamics::world::callbacks::ContactAccess<EntityUserData>,
        _: &dynamics::world::callbacks::ContactImpulse,
    ) {
    }
}

pub struct Player {
    pub ground_detector: Arc<RwLock<PlayerGroundDetector>>,
}

impl Player {
    pub fn create_physics(
        world: &mut p::World,
        pos: [f32; 2],
        size: [f32; 2],
        density: f32,
        player_ground_sensor_entity: Entity,
    ) -> p::Body {
        let body_desc = b2::BodyDef {
            body_type: b2::BodyType::Dynamic,
            position: b2::Vec2 {
                x: pos[0],
                y: pos[1],
            },

            ..b2::BodyDef::new()
        };
        let body_handle = world.world.create_body(&body_desc);
        let mut body_mut = world.world.body_mut(body_handle);

        let body_box_shape = b2::PolygonShape::new_oriented_box(
            size[0] * 0.49,
            (size[1] - size[0]) * 0.5,
            &b2::Vec2 {
                x: 0.0,
                y: size[1] * 0.5,
            },
            0.0,
        );
        let bottom_circle_shape = b2::CircleShape::new_with(
            b2::Vec2 {
                x: 0.0,
                y: size[0] * 0.5,
            },
            size[0] * 0.5,
        );
        let top_circle_shape = b2::CircleShape::new_with(
            b2::Vec2 {
                x: 0.0,
                y: size[1] - size[0] * 0.5,
            },
            size[0] * 0.5,
        );
        let jump_sensor_shape = b2::PolygonShape::new_box(size[0] * 0.2, size[0] * 0.2);

        let mut jump_sensor_fixture_def = dynamics::fixture::FixtureDef::new();
        jump_sensor_fixture_def.density = density;
        jump_sensor_fixture_def.is_sensor = true;

        body_mut.create_fast_fixture(&body_box_shape, density);
        body_mut.create_fast_fixture(&bottom_circle_shape, density);
        body_mut.create_fast_fixture(&top_circle_shape, density);
        body_mut.create_fixture_with(
            &jump_sensor_shape,
            &mut jump_sensor_fixture_def,
            player_ground_sensor_entity,
        );
        body_mut.set_rotation_fixed(true);

        p::Body {
            handle: body_handle,
        }
    }
}
