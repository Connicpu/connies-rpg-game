use {DataHelper, EntityIter};

#[derive(Default, System)]
#[system_type(Entity)]
#[process(process)]
#[aspect(all(player, transform))]
pub struct CameraFollow;

fn process(_: &mut CameraFollow, mut players: EntityIter, data: &mut DataHelper) {
    if let Some(player) = players.nth(0) {
        let player_pos = data.components.transform[player].pos;
        let cam_pos = data.services.camera.pos;
        let shifted = 0.8 * player_pos + 0.2 * cam_pos;
        data.services.camera.pos = shifted;
    }
}
