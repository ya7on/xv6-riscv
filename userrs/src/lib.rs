#![no_std]

extern crate alloc;

mod programs;
mod user;

#[cfg(not(test))]
mod xv6alloc {
    struct Xv6Alloc;

    unsafe impl core::alloc::GlobalAlloc for Xv6Alloc {
        unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
            unsafe {
                let p = super::user::syscalls::sbrk(layout.size() as i32);
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
            super::user::syscalls::write(1, b"panic\n".as_ptr(), 6);
            super::user::syscalls::exit(1);
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn hello_world_rs() -> i32 {
    programs::hello_world::main::<user::Xv6>()
}

#[unsafe(no_mangle)]
pub extern "C" fn calc_rs() -> i32 {
    programs::calc::main::<user::Xv6>()
}
