#![no_std]

extern crate alloc;

mod context;
mod io;
mod programs;
mod sys;

#[cfg(not(test))]
mod xv6alloc {
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
    fn panic(_: &core::panic::PanicInfo) -> ! {
        unsafe {
            sys::syscalls::write(1, b"panic\n".as_ptr(), 6);
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
