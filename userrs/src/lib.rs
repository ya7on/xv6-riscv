#![no_std]

mod programs;
mod user;

#[cfg(not(test))]
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn hello_world_rs() -> i32 {
    programs::hello_world::main::<user::Xv6>()
}

#[unsafe(no_mangle)]
pub extern "C" fn calc_rs() -> i32 {
    programs::calc::main::<user::Xv6>()
}
