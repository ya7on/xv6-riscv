use alloc::string::String;

use crate::{io::IO, sys::Sys};

pub fn main<X: Sys>(x: &X) -> i32 {
    let io = IO::new(x);

    let mut stdout_str = String::new();
    stdout_str.push_str("Hello, World!");
    io.println(&stdout_str);

    let mut stderr_str = String::new();
    stderr_str.push_str("World, Hello!");
    io.eprintln(&stderr_str);

    0
}
