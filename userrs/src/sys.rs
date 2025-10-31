/// Low level system calls
pub trait Sys {
    fn write(&self, output: &str) -> i32;
    fn writeln(&self, output: &str) -> i32;
    fn readln(&self, prompt: &str, buf: &mut [u8]) -> usize;
}

/// Safe guard before unsafe syscalls
/// Implementations of the Sys trait for the Xv6 system.
pub struct Xv6;

impl Sys for Xv6 {
    fn write(&self, output: &str) -> i32 {
        unsafe {
            syscalls::write(1, output.as_ptr(), output.len() as i32);
            0
        }
    }

    fn writeln(&self, output: &str) -> i32 {
        self.write(output);
        self.write("\n");
        0
    }

    fn readln(&self, prompt: &str, buf: &mut [u8]) -> usize {
        self.write(prompt);

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
