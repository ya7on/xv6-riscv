use crate::{context::Context, sys::Sys};

pub fn main<X: Sys>(ctx: Context<X>) -> i32 {
    let io = ctx.io();

    let args = ctx.args().as_slice();

    for arg in &args[1..] {
        io.print(arg);
        io.print(" ");
    }
    io.println("");

    0
}
