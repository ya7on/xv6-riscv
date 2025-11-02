use alloc::string::String;

use crate::{
    io::IO,
    sys::{FileDescriptor, Sys},
};

/// Main file mode.
#[derive(Clone, Copy)]
pub enum MainFileMode {
    /// Open a file for reading.
    /// `O_RDONLY`
    ReadOnly,
    // /// Open a file for writing.
    // /// `O_WRONLY`
    // WriteOnly,
    // /// Open a file for reading and writing.
    // /// `O_RDWR`
    // ReadWrite,
}

/// File mode.
#[derive(Clone, Copy)]
pub struct FileMode {
    /// Main file mode.
    pub main: MainFileMode,
    /// Create the file if it does not exist.
    /// `O_CREATE`
    pub create: bool,
    /// Truncate the file before opening.
    /// `O_TRUNC`
    pub truncate: bool,
}

impl FileMode {
    /// Simple read-only mode.
    pub fn read() -> Self {
        Self {
            main: MainFileMode::ReadOnly,
            create: false,
            truncate: false,
        }
    }

    pub fn can_read(&self) -> bool {
        match self.main {
            MainFileMode::ReadOnly => true,
            // MainFileMode::WriteOnly => false,
            // MainFileMode::ReadWrite => true,
        }
    }
}

impl Into<i32> for FileMode {
    fn into(self) -> i32 {
        let mut flags = 0;
        match self.main {
            MainFileMode::ReadOnly => flags |= 0x000,
            // MainFileMode::WriteOnly => flags |= 0x001,
            // MainFileMode::ReadWrite => flags |= 0x002,
        }
        if self.create {
            flags |= 0x200;
        }
        if self.truncate {
            flags |= 0x400;
        }
        flags
    }
}

/// High-level file descriptor wrapper.
pub struct File<'a, X: Sys> {
    sys: &'a X,
    mode: FileMode,
    fd: FileDescriptor,
}

impl<'a, X: Sys> File<'a, X> {
    pub fn new(sys: &'a X, fd: FileDescriptor, mode: FileMode) -> Self {
        Self { sys, mode, fd }
    }

    // TODO: Add lazy reading support

    /// Read a full file into memory.
    pub fn read(&self) -> String {
        if !self.mode.can_read() {
            panic!("File is not readable");
        }
        let io = IO::new(self.sys);
        let mut result = String::new();
        loop {
            let read = io.read::<1024>(self.fd);
            result.push_str(&read);
            if read.is_empty() {
                break;
            }
        }
        result
    }
}

impl<'a, X: Sys> Drop for File<'a, X> {
    fn drop(&mut self) {
        self.sys.close(self.fd);
    }
}
