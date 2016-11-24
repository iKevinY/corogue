#![feature(lang_items)]
#![no_std]
#![no_main]

extern crate gba;

mod lang;

pub use lang::{__aeabi_unwind_cpp_pr0, __aeabi_unwind_cpp_pr1};

use gba::gfx;
use gba::gfx::Color;
use gba::input;


#[no_mangle]
pub extern "C" fn main(_: i32, _: *const *const i8) -> i32 {
    let width = gfx::Mode3::WIDTH as i32;
    let height = gfx::Mode3::HEIGHT as i32;

    let mut m = gfx::Mode3::new();
    let mut keys = input::Input::new();

    let mut x = width / 2;
    let mut y = height / 2;

    loop {
        // Only draw once per frame
        gfx::vid_vsync();

        // Save current state of keypresses
        keys.poll();

        x += keys.tri_horz();
        y += keys.tri_vert();

        // Keep cursor within screen (wrap around edges)
        x = (x + width) % width;
        y = (y + height) % height;

        // Draw a dot
        m.dot(x, y, Color::rgb15(31, 31, 31));
    }
}
