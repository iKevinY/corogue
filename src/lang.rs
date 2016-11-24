use core;

#[lang = "panic_fmt"]
extern "C" fn panic_fmt(_args: &core::fmt::Arguments, _file: &str, _line: u32) -> ! {
    loop {}
}

#[lang = "eh_personality"]
pub extern "C" fn eh_personality() -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn __aeabi_unwind_cpp_pr0() {
    loop {}
}

#[no_mangle]
pub extern "C" fn __aeabi_unwind_cpp_pr1() {
    loop {}
}
