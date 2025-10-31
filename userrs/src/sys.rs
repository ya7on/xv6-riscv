/// File descriptor for system calls
#[derive(Clone, Copy)]
pub enum FileDescriptor {
    Stdin,
    Stdout,
    Stderr,
}

impl Into<i32> for FileDescriptor {
    fn into(self) -> i32 {
        match self {
            FileDescriptor::Stdin => 0,
            FileDescriptor::Stdout => 1,
            FileDescriptor::Stderr => 2,
        }
    }
}

/// Low level system calls
pub trait Sys {
    fn write(&self, fd: FileDescriptor, output: &str) -> i32;
    fn writeln(&self, fd: FileDescriptor, output: &str) -> i32;
    fn readln(&self, fd: FileDescriptor, buf: &mut [u8]) -> usize;
}

/// Safe guard before unsafe syscalls
/// Implementations of the Sys trait for the Xv6 system.
pub struct Xv6;

impl Sys for Xv6 {
    fn write(&self, fd: FileDescriptor, output: &str) -> i32 {
        unsafe {
            syscalls::write(fd.into(), output.as_ptr(), output.len() as i32);
            0
        }
    }

    fn writeln(&self, fd: FileDescriptor, output: &str) -> i32 {
        self.write(fd, output);
        self.write(fd, "\n");
        0
    }

    fn readln(&self, fd: FileDescriptor, buf: &mut [u8]) -> usize {
        let mut i = 0;

        loop {
            let mut ch: [u8; 1] = [0];
            unsafe {
                syscalls::read(fd.into(), ch.as_mut_ptr(), 1);
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
