#[derive(Debug)]
pub struct Timer {
    pub delta_time: f32,
    pub running_time: f64,

    pub prev_time: u64,
    pub time: u64,
}
