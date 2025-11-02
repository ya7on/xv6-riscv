use crate::{context::Context, file::FileMode, sys::Sys};

pub fn main<X: Sys>(ctx: Context<X>) -> i32 {
    let io = ctx.io();

    if ctx.args().len() < 2 {
        io.eprintln("Usage: cat <file>");
        return 1;
    }

    let args = ctx.args().as_slice();

    for filename in &args[1..] {
        let file = io
            .open(filename, FileMode::read())
            .expect("Failed to open file");

        let content = file.read();
        io.println(&content);
    }

    0
}
