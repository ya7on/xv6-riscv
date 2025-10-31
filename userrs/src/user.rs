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

/// Kernel system calls
pub mod syscalls {
    unsafe extern "C" {
        /// Write to a file descriptor
        pub unsafe fn write(fd: i32, buf: *const u8, len: i32) -> i32;
        /// Read from a file descriptor
        pub unsafe fn read(fd: i32, buf: *mut u8, len: i32) -> i32;
        /// Allocate memory
        #[allow(dead_code)]
        pub unsafe fn sbrk(n: i32) -> *mut u8;
        /// Exit the process
        #[allow(dead_code)]
        pub unsafe fn exit(status: i32) -> !;
    }
}
