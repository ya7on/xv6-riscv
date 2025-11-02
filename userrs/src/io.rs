use alloc::string::String;

use crate::{
    file::{File, FileMode},
    sys::{FileDescriptor, Sys},
};

/// High-level interface for interacting with the IO.
pub struct IO<'a, X: Sys> {
    sys: &'a X, // Not sure does it need to be pointer
}

impl<'a, X: Sys> IO<'a, X> {
    /// Create a new IO instance.
    pub fn new(sys: &'a X) -> Self {
        IO { sys }
    }

    /// Read a full string from the descriptor.
    /// Allocate a buffer and read a full input
    pub fn read<const BUF_SIZE: usize>(&self, fd: FileDescriptor) -> String {
        let mut buf = [0u8; BUF_SIZE];
        let len = self.sys.read(fd, &mut buf);

        let s = core::str::from_utf8(&buf[..len]).unwrap_or("");
        String::from(s)
    }

    /// Read a line from the descriptor.
    /// Allocate a buffer and read a line from the user input.
    pub fn readln<const BUF_SIZE: usize>(&self, fd: FileDescriptor) -> String {
        let mut buf = [0u8; BUF_SIZE];
        let len = self.sys.readln(fd, &mut buf);

        let s = core::str::from_utf8(&buf[..len]).unwrap_or("");
        String::from(s)
    }

    /// Read a line from the user input.
    /// Allocate a buffer and read a line from the user input.
    /// Use default buffer size.
    pub fn input(&self, prompt: &str) -> String {
        self.sys.write(FileDescriptor::Stdout, prompt);
        self.readln::<128>(FileDescriptor::Stdin)
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
    pub fn open(&self, path: &str, mode: FileMode) -> File<'a, X> {
        let fd = self.sys.open(path, mode.into());
        File::new(self.sys, FileDescriptor::Custom(fd), mode)
    }
}
