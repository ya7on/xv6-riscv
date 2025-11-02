use alloc::string::String;

use crate::{
    io::IO,
    sys::{FileDescriptor, Sys},
};

pub mod flags {
    pub const O_RDONLY: i32 = 0x000;
    pub const O_WRONLY: i32 = 0x001;
    // pub const O_RDWR: i32 = 0x002;
    pub const O_CREATE: i32 = 0x200;
    pub const O_TRUNC: i32 = 0x400;
}

/// Main file mode.
#[derive(Clone, Copy)]
pub enum MainFileMode {
    /// Open a file for reading.
    /// `O_RDONLY`
    ReadOnly,
    /// Open a file for writing.
    /// `O_WRONLY`
    WriteOnly,
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

    /// Write mode with creating file
    pub fn create() -> Self {
        Self {
            main: MainFileMode::WriteOnly,
            create: true,
            truncate: false,
        }
    }

    pub fn can_read(&self) -> bool {
        match self.main {
            MainFileMode::ReadOnly => true,
            MainFileMode::WriteOnly => false,
            // MainFileMode::ReadWrite => true,
        }
    }
}

impl Into<i32> for FileMode {
    fn into(self) -> i32 {
        let mut flags = 0;
        match self.main {
            MainFileMode::ReadOnly => flags |= flags::O_RDONLY,
            MainFileMode::WriteOnly => flags |= flags::O_WRONLY,
            // MainFileMode::ReadWrite => flags |= flags::O_RDWR,
        }
        if self.create {
            flags |= flags::O_CREATE;
        }
        if self.truncate {
            flags |= flags::O_TRUNC;
        }
        flags
    }
}

/// High-level file descriptor wrapper.
pub struct File<'a, X: Sys> {
    io: &'a IO<'a, X>,
    mode: FileMode,
    fd: FileDescriptor,
}

impl<'a, X: Sys> File<'a, X> {
    pub fn new(io: &'a IO<'a, X>, fd: FileDescriptor, mode: FileMode) -> Self {
        Self { io, mode, fd }
    }

    // TODO: Add lazy reading support

    /// Read a full file into memory.
    pub fn read(&self) -> String {
        if !self.mode.can_read() {
            panic!("File is not readable");
        }

        let mut result = String::new();
        let mut buf = [0u8; 1024];
        loop {
            let read = self
                .io
                .read_into(self.fd, &mut buf)
                .expect("Cannot read file");
            if read == 0 {
                break;
            }

            if let Ok(s) = core::str::from_utf8(&buf[..(read as usize)]) {
                result.push_str(s);
            } else {
                panic!("File is not text");
            }
        }
        result
    }
}

impl<'a, X: Sys> Drop for File<'a, X> {
    fn drop(&mut self) {
        self.io.close(self.fd);
    }
}
