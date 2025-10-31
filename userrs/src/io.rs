use alloc::string::String;

use crate::sys::{FileDescriptor, Sys};

/// High-level interface for interacting with the IO.
pub struct IO<'a, X: Sys> {
    sys: &'a X, // Not sure does it need to be pointer
}

impl<'a, X: Sys> IO<'a, X> {
    /// Create a new IO instance.
    pub fn new(sys: &'a X) -> Self {
        IO { sys }
    }

    /// Read a line from the user input.
    /// Allocate a buffer and read a line from the user input.
    pub fn input_sized<const BUF_SIZE: usize>(&self, prompt: &str) -> String {
        self.sys.write(FileDescriptor::Stdout, prompt);

        let mut buf = [0u8; BUF_SIZE];
        let len = self.sys.readln(FileDescriptor::Stdin, &mut buf);

        let s = core::str::from_utf8(&buf[..len]).unwrap_or("");
        String::from(s)
    }

    /// Read a line from the user input.
    /// Allocate a buffer and read a line from the user input.
    /// Use default buffer size.
    pub fn input(&self, prompt: &str) -> String {
        self.input_sized::<128>(prompt)
    }

    /// Print a message to the STDOUT output.
    pub fn println(&self, msg: &str) {
        self.sys.writeln(FileDescriptor::Stdout, msg);
    }

    /// Print a message to the STDERR output.
    pub fn eprintln(&self, msg: &str) {
        self.sys.writeln(FileDescriptor::Stderr, msg);
    }
}
