use core::fmt::Debug;

use alloc::string::String;

use crate::{
    file::{File, FileMode},
    sys::{FileDescriptor, Sys},
};

pub type IoResult<R> = Result<R, IoError>;

pub enum IoError {
    // TODO: Add errno to kernel
    Unknown(i32),
}

impl Debug for IoError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            IoError::Unknown(code) => write!(f, "Unknown error: {}", code),
        }
    }
}

impl IoError {
    fn match_read_error(code: i32) -> Self {
        match code {
            _ => IoError::Unknown(code),
        }
    }

    fn match_open_error(code: i32) -> Self {
        match code {
            _ => IoError::Unknown(code),
        }
    }
}

/// High-level interface for interacting with the IO.
pub struct IO<'a, X: Sys> {
    sys: &'a X, // Not sure does it need to be pointer
}

impl<'a, X: Sys> IO<'a, X> {
    /// Create a new IO instance.
    pub fn new(sys: &'a X) -> Self {
        IO { sys }
    }

    /// Read data into a buffer from a file descriptor.
    pub fn read_into(&self, fd: FileDescriptor, buf: &mut [u8]) -> IoResult<i32> {
        let code = self.sys.read(fd, buf);
        if code < 0 {
            return Err(IoError::match_read_error(code as i32));
        }
        Ok(code)
    }

    // /// Read a full string from the descriptor.
    // /// Allocate a buffer and read a full input
    // pub fn read_string<const BUF_SIZE: usize>(&self, fd: FileDescriptor) -> String {
    //     let mut buf = [0u8; BUF_SIZE];
    //     let len = self.sys.read(fd, &mut buf);

    //     let s = core::str::from_utf8(&buf[..len]).unwrap_or("");
    //     String::from(s)
    // }

    /// Read data into a buffer from a file descriptor.
    pub fn readln_into(&self, fd: FileDescriptor, buf: &mut [u8]) -> IoResult<i32> {
        let code = self.sys.readln(fd, buf);
        if code < 0 {
            return Err(IoError::match_read_error(code as i32));
        }
        Ok(code)
    }

    /// Read a line from the descriptor.
    /// Allocate a buffer and read a line from the user input.
    pub fn readln_string<const BUF_SIZE: usize>(&self, fd: FileDescriptor) -> IoResult<String> {
        let mut buf = [0u8; BUF_SIZE];
        let len = self.readln_into(fd, &mut buf)?;

        let s = core::str::from_utf8(&buf[..(len as usize)]).unwrap_or("");
        Ok(String::from(s))
    }

    /// Read a line from the user input.
    /// Allocate a buffer and read a line from the user input.
    /// Use default buffer size.
    pub fn input(&self, prompt: &str) -> IoResult<String> {
        self.sys.write(FileDescriptor::Stdout, prompt);
        self.readln_string::<128>(FileDescriptor::Stdin)
    }

    /// Print a message to the STDOUT output without a newline at the end.
    pub fn print(&self, msg: &str) {
        self.sys.write(FileDescriptor::Stdout, msg);
    }

    /// Print a message to the STDOUT output with a newline at the end.
    pub fn println(&self, msg: &str) {
        self.sys.writeln(FileDescriptor::Stdout, msg);
    }

    /// Print a message to the STDERR output with a newline at the end.
    pub fn eprintln(&self, msg: &str) {
        self.sys.writeln(FileDescriptor::Stderr, msg);
    }

    // /// Open a file for reading or writing.
    pub fn open(&'a self, path: &str, mode: FileMode) -> IoResult<File<'a, X>> {
        let code = self.sys.open(path, mode.into());
        if code < 0 {
            return Err(IoError::match_open_error(code as i32));
        }
        Ok(File::new(self, FileDescriptor::Custom(code), mode))
    }

    pub fn close(&self, fd: FileDescriptor) {
        self.sys.close(fd);
    }
}
