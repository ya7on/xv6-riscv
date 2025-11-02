#![no_std]

#[unsafe(no_mangle)]
pub extern "C" fn rust_hello() {
    let msg = b"Rust kernel module started!\n";
    unsafe {
        unsafe extern "C" {
            fn uartputc_sync(c: i32);
        }
        for &ch in msg {
            uartputc_sync(ch as i32);
        }
    }
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
