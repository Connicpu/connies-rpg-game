use DataHelper;

#[derive(Default, System)]
#[process(update)]
pub struct AudioUpdate;

fn update(_: &mut AudioUpdate, data: &mut DataHelper) {
    data.services
        .audio
        .studio
        .update()
        .expect("FMOD Studio updating shouldn't fail");
}
