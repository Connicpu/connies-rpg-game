#[derive(Default, System)]
#[process(process)]
pub struct PhysicsRun;

fn process(_: &mut PhysicsRun, data: &mut ::DataHelper) {
    let dt = data.services.timer.delta_time;
    data.services.physics.world.step(dt, 16, 6);
}
