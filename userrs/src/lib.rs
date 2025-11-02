#![no_std]

extern crate alloc;

mod context;
mod file;
mod io;
mod programs;
mod sys;

#[cfg(not(test))]
mod xv6alloc {
    use alloc::format;

    use super::sys;

    struct Xv6Alloc;

    unsafe impl core::alloc::GlobalAlloc for Xv6Alloc {
        unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
            unsafe {
                let p = sys::syscalls::sbrk(layout.size() as i32);
                if p as isize == -1 {
                    core::ptr::null_mut()
                } else {
                    p
                }
            }
        }

        unsafe fn dealloc(&self, _ptr: *mut u8, _layout: core::alloc::Layout) {}
    }

    #[global_allocator]
    static A: Xv6Alloc = Xv6Alloc;

    #[panic_handler]
    fn panic(info: &core::panic::PanicInfo) -> ! {
        unsafe {
            sys::syscalls::write(1, b"=== RUST PANIC in xv6 ===\n".as_ptr(), 26);
            let msg = format!("{}", info);
            sys::syscalls::write(1, msg.as_ptr(), msg.len() as i32);
            sys::syscalls::write(1, b"\n".as_ptr(), 1);

            sys::syscalls::exit(1);
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn hello_world_rs() -> i32 {
    let x = sys::Xv6;
    let context = context::Context::new(&x);
    programs::hello_world::main(context)
}

#[unsafe(no_mangle)]
pub extern "C" fn calc_rs() -> i32 {
    let x = sys::Xv6;
    let context = context::Context::new(&x);
    programs::calc::main(context)
}

#[unsafe(no_mangle)]
pub extern "C" fn echo_rs(argc: i32, argv: *const *const u8) -> i32 {
    let x = sys::Xv6;
    let args = unsafe { context::Args::from_raw(argc, argv) };
    let context = context::Context::new_with_args(&x, args);
    programs::echo::main(context)
}

#[unsafe(no_mangle)]
pub extern "C" fn cat_rs(argc: i32, argv: *const *const u8) -> i32 {
    let x = sys::Xv6;
    let args = unsafe { context::Args::from_raw(argc, argv) };
    let context = context::Context::new_with_args(&x, args);
    programs::cat::main(context)
}

#[unsafe(no_mangle)]
pub extern "C" fn touch_rs(argc: i32, argv: *const *const u8) -> i32 {
    let x = sys::Xv6;
    let args = unsafe { context::Args::from_raw(argc, argv) };
    let context = context::Context::new_with_args(&x, args);
    programs::touch::main(context)
}
