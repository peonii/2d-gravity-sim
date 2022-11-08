use raylib::prelude::*;

use crate::{constants::{HEIGHT, CONFIG, HISTORY}, conf::simcfg::SimulatorConfig};
use anyhow::{anyhow, Result};

use super::pos::Position;

pub struct CollisionBox {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    yvel: i32,
    mass: i32,
    gravity: i32
}

impl Default for CollisionBox {
    fn default() -> Self {
        let config = CONFIG.lock().unwrap(); // it shouldn't panic... right???

        Self {
            x: config.x,
            y: config.y,
            width: config.w,
            height: config.h,
            yvel: config.yv,
            mass: config.mass,
            gravity: config.gravity
        }
    }
}

impl CollisionBox {
    pub fn new(cfg: &SimulatorConfig) -> Self {
        Self {
            x: cfg.x,
            y: cfg.y,
            width: cfg.w,
            height: cfg.h,
            yvel: cfg.yv,
            mass: cfg.mass,
            gravity: cfg.gravity
        }
    }

    pub fn draw(&mut self, dh: &mut RaylibDrawHandle) {
        //dh.draw_rectangle(self.x, self.y, self.width, self.height, Color::BLACK);
        // lmao
    }

    pub fn update(&mut self, _dh: &mut RaylibDrawHandle) -> Result<()> {
        self.y -= self.yvel;
        self.yvel += self.gravity * self.mass;
        self.x += 6;

        let mut positions = match HISTORY.lock() {
            Ok(o) => o,
            Err(_) => return Err(anyhow!("Error locking HISTORY mutex!")),
        };

        if self.y + self.height > HEIGHT {
            self.y = HEIGHT - self.height;
        } else {
            positions.push(Position::new(self.x, self.y));
        }

        Ok(())
    }
}
