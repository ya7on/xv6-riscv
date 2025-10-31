use alloc::vec::Vec;

use crate::{io::IO, sys::Sys};

pub struct Args<'a>(Vec<&'a str>);

impl<'a> Args<'a> {
    /// Create a new Args instance from a vector of strings.
    pub fn new(args: Vec<&'a str>) -> Self {
        Args(args)
    }

    /// Create a new Args instance from raw C-style arguments.
    pub unsafe fn from_raw(argc: i32, argv: *const *const u8) -> Args<'a> {
        unsafe fn c_strlen(mut p: *const u8) -> usize {
            let mut n = 0;
            while unsafe { *p } != 0 {
                n += 1;
                p = unsafe { p.add(1) };
            }
            n
        }

        let mut v = Vec::with_capacity(argc as usize);
        for i in 0..argc {
            let p = unsafe { *argv.add(i as usize) };
            if p.is_null() {
                continue;
            }
            let len = unsafe { c_strlen(p) };
            let slice = unsafe { core::slice::from_raw_parts(p, len) };
            let s = unsafe { core::str::from_utf8_unchecked(slice) };
            v.push(s);
        }
        Args(v)
    }

    /// Returns a slice of the command-line arguments.
    pub fn as_slice(&self) -> &[&str] {
        &self.0
    }
}

pub struct Context<'a, X: Sys> {
    /// High-level client to IO
    io: IO<'a, X>,
    /// Command-line arguments
    args: Args<'a>,
}

impl<'a, X: Sys> Context<'a, X> {
    pub fn new(sys: &'a X) -> Self {
        Context {
            io: IO::new(sys),
            args: Args::new(Vec::new()),
        }
    }

    pub fn new_with_args(sys: &'a X, args: Args<'a>) -> Self {
        Context {
            io: IO::new(sys),
            args,
        }
    }

    /// Get pointer to IO client
    pub fn io(&self) -> &IO<'a, X> {
        &self.io
    }

    /// Get pointer to command-line arguments
    pub fn args(&self) -> &Args<'a> {
        &self.args
    }
}
