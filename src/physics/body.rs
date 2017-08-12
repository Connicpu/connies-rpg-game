use wrapped2d::b2::BodyHandle;
use specs::VecStorage;

#[derive(Component)]
#[component(VecStorage)]
pub struct Body {
    pub handle: BodyHandle,
}
