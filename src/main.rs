#![feature(lang_items)]
#![no_std]
#![no_main]

extern crate gba;

mod lang;
mod rand;

pub use lang::{__aeabi_unwind_cpp_pr0, __aeabi_unwind_cpp_pr1};

use gba::gfx;
use gba::input::Keys;

use rand::Rng;


#[no_mangle]
pub extern "C" fn main(_: i32, _: *const *const i8) -> i32 {
    let mut m = gfx::Mode3::new();
    let mut keys = gba::input::Input::new();

    let width = gfx::Mode3::WIDTH as i32;
    let height = gfx::Mode3::HEIGHT as i32;

    let mut rng = Rng::new(42);

    loop {
        // Only draw once per frame
        gfx::vid_vsync();

        // Clear screen if the A button is pressed
        keys.poll();

        if keys.hit(Keys::A) {
            let black = gfx::Color::rgb15(0, 0, 0);

            for y in 0..height {
                for x in 0..width {
                    m.dot(x, y, black);
                }
            }
        }

        // Draw 64 dots per frame
        for _ in 0..64 {
            let x = rng.randint(0, width as u32) as i32;
            let y = rng.randint(0, height as u32) as i32;

            let color = gfx::Color::rgb15(
                rng.randint(0, 31),
                rng.randint(0, 31),
                rng.randint(0, 31),
            );

            m.dot(x, y, color);
        }
    }
}
