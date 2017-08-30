use DataHelper;
use util::Mutate;

#[derive(Default, System)]
#[process(process)]
pub struct LockCamera;

fn process(_: &mut LockCamera, data: &mut DataHelper) {
    let aspect = data.services.camera.aspect_ratio;
    data.services.camera.pos.mutate(|cpos| {
        cpos.y = cpos.y.max(-252.0).min(252.0);
        cpos.x = cpos.x.max(4.0 * aspect).min(256.0 - 4.0 * aspect);
    });
}
