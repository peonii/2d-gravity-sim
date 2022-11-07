use std::sync::Mutex;

use once_cell::sync::Lazy;

use crate::object::collision_box::CollisionBox;

pub static WIDTH: i32 = 800;
pub static HEIGHT: i32 = 600;

pub static OBJECT: Lazy<Mutex<CollisionBox>> = Lazy::new(|| Mutex::new(CollisionBox::default()));

pub static PLAYING: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));
