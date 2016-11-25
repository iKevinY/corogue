#![feature(lang_items)]
#![no_std]
#![no_main]

extern crate gba;

mod lang;
mod rand;

pub use lang::{__aeabi_unwind_cpp_pr0, __aeabi_unwind_cpp_pr1};

use gba::gfx::vid_vsync;
use gba::gfx::{Color, Mode3};

use rand::Rng;


#[no_mangle]
pub extern "C" fn main(_: i32, _: *const *const i8) -> i32 {
    let mut m = Mode3::new();
    let width = Mode3::WIDTH as i32;
    let height = Mode3::HEIGHT as i32;

    let mut rng = Rng::new(42);

    loop {
        // Only draw once per frame
        vid_vsync();

        // Generate a random coordinate
        let x = rng.randint(0, width as u32) as i32;
        let y = rng.randint(0, height as u32) as i32;

        // Generate a random colour
        let color = Color::rgb15(
            rng.randint(0, 31),
            rng.randint(0, 31),
            rng.randint(0, 31),
        );

        // Draw dot to screen
        m.dot(x, y, color);
    }
}
