/// File descriptor for system calls
#[derive(Clone, Copy)]
pub enum FileDescriptor {
    /// Standard input (0 descriptor)
    Stdin,
    /// Standard output (1 descriptor)
    Stdout,
    /// Standard error (2 descriptor)
    Stderr,
    /// Custom file descriptor
    Custom(i32),
}

impl From<FileDescriptor> for i32 {
    fn from(fd: FileDescriptor) -> Self {
        match fd {
            FileDescriptor::Stdin => 0,
            FileDescriptor::Stdout => 1,
            FileDescriptor::Stderr => 2,
            FileDescriptor::Custom(fd) => fd,
        }
    }
}

impl From<i32> for FileDescriptor {
    fn from(fd: i32) -> Self {
        match fd {
            0 => FileDescriptor::Stdin,
            1 => FileDescriptor::Stdout,
            2 => FileDescriptor::Stderr,
            _ => FileDescriptor::Custom(fd),
        }
    }
}

/// Low level system calls
pub trait Sys {
    /// Write to a file descriptor
    fn write(&self, fd: FileDescriptor, output: &str) -> i32;
    /// Write to a file descriptor with a newline
    fn writeln(&self, fd: FileDescriptor, output: &str) -> i32;
    /// Read from a file descriptor
    fn read(&self, fd: FileDescriptor, buf: &mut [u8]) -> i32;
    /// Read from a file descriptor until a newline is encountered
    fn readln(&self, fd: FileDescriptor, buf: &mut [u8]) -> i32;
    /// Open a file descriptor
    fn open(&self, path: &str, flags: i32) -> i32;
    /// Close a file descriptor
    fn close(&self, fd: FileDescriptor) -> i32;
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

    fn read(&self, fd: FileDescriptor, buf: &mut [u8]) -> i32 {
        let mut i = 0;
        loop {
            let mut ch: [u8; 1] = [0];
            unsafe {
                let response = syscalls::read(fd.into(), ch.as_mut_ptr(), 1);
                if response < 0 {
                    return response;
                }
            }
            if ch[0] == 0 || i + 1 >= buf.len() {
                buf[i] = 0;
                break;
            }
            buf[i] = ch[0];
            i += 1;
        }
        i as i32
    }

    fn readln(&self, fd: FileDescriptor, buf: &mut [u8]) -> i32 {
        let mut i = 0;

        loop {
            let mut ch: [u8; 1] = [0];
            unsafe {
                let response = syscalls::read(fd.into(), ch.as_mut_ptr(), 1);
                if response < 0 {
                    return response;
                }
            }
            if ch[0] == b'\n' || ch[0] == 0 || i + 1 >= buf.len() {
                buf[i] = 0;
                break;
            }
            buf[i] = ch[0];
            i += 1;
        }

        i as i32
    }

    fn open(&self, path: &str, flags: i32) -> i32 {
        unsafe { syscalls::open(path.as_ptr(), flags) }
    }

    fn close(&self, fd: FileDescriptor) -> i32 {
        unsafe { syscalls::close(fd.into()) }
    }
}

/// Kernel system calls
pub mod syscalls {
    unsafe extern "C" {
        /// Write to a file descriptor
        pub unsafe fn write(fd: i32, buf: *const u8, len: i32) -> i32;
        /// Read from a file descriptor
        pub unsafe fn read(fd: i32, buf: *mut u8, len: i32) -> i32;
        /// Open a file descriptor
        pub unsafe fn open(path: *const u8, flags: i32) -> i32;
        /// Close a file descriptor
        pub unsafe fn close(fd: i32) -> i32;
        /// Allocate memory
        #[allow(dead_code)]
        pub unsafe fn sbrk(n: i32) -> *mut u8;
        /// Exit the process
        #[allow(dead_code)]
        pub unsafe fn exit(status: i32) -> !;
    }
}
