pub trait UserAPI {
    fn write(output: &str) -> i32;
    fn writeln(output: &str) -> i32;
    fn readln(prompt: &str, buf: &mut [u8]) -> usize;
}

/// Safe guard before unsafe syscalls and user API calls
pub struct Xv6;

impl UserAPI for Xv6 {
    fn write(output: &str) -> i32 {
        unsafe {
            syscalls::write(1, output.as_ptr(), output.len() as i32);
            0
        }
    }

    fn writeln(output: &str) -> i32 {
        unsafe {
            syscalls::write(1, output.as_ptr(), output.len() as i32);
            syscalls::write(1, b"\n".as_ptr(), 1);
            0
        }
    }

    fn readln(prompt: &str, buf: &mut [u8]) -> usize {
        Self::write(prompt);

        let mut i = 0;

        loop {
            let mut ch: [u8; 1] = [0];
            unsafe {
                syscalls::read(0, ch.as_mut_ptr(), 1);
            }
            if ch[0] == b'\n' || ch[0] == 0 || i + 1 >= buf.len() {
                buf[i] = 0;
                break;
            }
            buf[i] = ch[0];
            i += 1;
        }

        i
    }
}

// user.h ffi

pub(crate) mod syscalls {
    unsafe extern "C" {
        pub(crate) unsafe fn write(fd: i32, buf: *const u8, len: i32) -> i32;
        pub(crate) unsafe fn read(fd: i32, buf: *mut u8, len: i32) -> i32;
    }
}
pub(crate) mod ulib {
    unsafe extern "C" {
        // pub(crate) unsafe fn atoi(s: *const u8) -> i32;
        // pub(crate) unsafe fn memset(buf: *mut u8, c: u8, len: i32) -> *mut u8;
        // pub(crate) unsafe fn gets(buf: *mut u8, len: i32) -> *mut u8;
        // pub(crate) unsafe fn strlen(buf: *const u8) -> u32;
        // pub(crate) unsafe fn strcmp(a: *const u8, b: *const u8) -> i32;
    }
}
pub(crate) mod print {
    unsafe extern "C" {
        // pub(crate) unsafe fn printf(fmt: *const u8, ...) -> i32;
    }
}
