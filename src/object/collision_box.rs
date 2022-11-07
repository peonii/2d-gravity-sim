use raylib::prelude::*;

use crate::constants::HEIGHT;

pub struct CollisionBox {
    x: i32,
    y: i32,
    width: i32,
    height: i32,

    yaccel: i32,
}

impl Default for CollisionBox {
    fn default() -> Self {
        Self {
            x: 400,
            y: 0,
            width: 50,
            height: 50,
            yaccel: 0,
        }
    }
}

impl CollisionBox {
    pub fn draw(&mut self, dh: &mut RaylibDrawHandle) {
        dh.draw_rectangle(self.x, self.y, self.width, self.height, Color::BLACK);
    }

    pub fn update(&mut self, dh: &mut RaylibDrawHandle) {
        self.draw(dh);

        self.y += self.yaccel;
        self.yaccel += 1;

        if self.y + self.height > HEIGHT {
            self.y = HEIGHT - self.height;
        }
    }
}
