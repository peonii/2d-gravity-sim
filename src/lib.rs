use anyhow::{anyhow, Result};
use constants::{OBJECT, PLAYING};
use raylib::prelude::*;

pub mod constants;
pub mod object;

pub fn update(rl: &mut RaylibHandle, thread: &RaylibThread) -> Result<()> {
    let mut drawing_handle = rl.begin_drawing(thread);

    drawing_handle.clear_background(Color::WHITE);
    drawing_handle.draw_text("Hello, world!", 10, 10, 20, Color::BLACK);

    let mut playing = match PLAYING.lock() {
        Ok(o) => o,
        Err(_) => return Err(anyhow!("Error locking PLAYING mutex!")),
    };

    drawing_handle.gui_check_box(rrect(10, 10, 60, 60), Some(rstr!("Playing")));

    let mut obj = match OBJECT.lock() {
        Ok(o) => o,
        Err(_) => return Err(anyhow!("Error locking OBJECT mutex!")),
    };

    obj.update(&mut drawing_handle);

    Ok(())
}
