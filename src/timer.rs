use time;

#[derive(Debug)]
pub struct Timer {
    pub delta_time: f32,
    pub running_time: f64,

    pub start_time: u64,
    pub time: u64,
    pub delta_time_ns: u64,
}

impl Timer {
    pub fn new() -> Self {
        let now = time::precise_time_ns();

        Timer {
            delta_time: 0.0,
            running_time: 0.0,
            start_time: now,
            time: now,
            delta_time_ns: 0,
        }
    }

    pub fn update(&mut self) {
        let now = time::precise_time_ns();
        let diff_time = now - self.time;
        let running_time = now - self.start_time;

        // If we process the floats in nanoseconds we'll be wrecking our f32 precision.
        // 100ns resolution is plenty.
        self.delta_time = (diff_time / 100) as f32 / 1.0e7;
        self.running_time = running_time as f64 / 1.0e9;

        self.delta_time_ns = diff_time;
        self.time = now;
    }

    pub fn immediate_frametime_ns(&self) -> u64 {
        let now = time::precise_time_ns();

        now - self.time
    }
}

impl Default for Timer {
    fn default() -> Self {
        Timer::new()
    }
}

#[derive(Default, System)]
#[process = "UpdateTime::process"]
pub struct UpdateTime;

impl UpdateTime {
    fn process(&mut self, data: &mut ::DataHelper) {
        data.services.timer.update();
    }
}
