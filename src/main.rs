use anyhow::Result;
use raylib::prelude::*;
use sym_swobodne::constants::{HEIGHT, WIDTH};
use sym_swobodne::update;

fn main() -> Result<()> {
    let (mut rl, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("Symulacja")
        .build();

    rl.set_target_fps(60);

    while !rl.window_should_close() {
        update(&mut rl, &thread)?;
    }

    Ok(())
}
