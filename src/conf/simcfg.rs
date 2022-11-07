
pub struct SimulatorConfig {
    pub playing: bool,
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
    pub yv: i32,
    pub mass: i32,
    pub gravity: i32
}

impl Default for SimulatorConfig {
    fn default() -> Self {
        Self { playing: false, x: 800, y: 1200, w: 25, h: 25, yv: 0, mass: 1, gravity: 1 }
    }
}