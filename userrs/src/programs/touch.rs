use crate::{context::Context, file::FileMode, sys::Sys};

pub fn main<X: Sys>(ctx: Context<X>) -> i32 {
    let io = ctx.io();

    if ctx.args().len() < 2 {
        io.println("Usage: touch <file>");
        return 1;
    }

    let args = ctx.args().as_slice();

    for filename in &args[1..] {
        io.open(filename, FileMode::create());
    }

    0
}
