use alloc::string::String;

use crate::{context::Context, sys::Sys};

pub fn main<X: Sys>(ctx: Context<X>) -> i32 {
    let io = ctx.io();

    let mut stdout_str = String::new();
    stdout_str.push_str("Hello, World!");
    io.println(&stdout_str);

    let mut stderr_str = String::new();
    stderr_str.push_str("World, Hello!");
    io.eprintln(&stderr_str);

    0
}
