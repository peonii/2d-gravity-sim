pub struct SimPoint {
    pub y: f64,
    pub vel: f64,
    pub time: f64,
}

impl SimPoint {
    pub fn new(time: f64, y: f64, vel: f64) -> Self {
        Self { y, vel, time }
    }
}
