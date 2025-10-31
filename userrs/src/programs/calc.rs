use alloc::format;

use crate::{context::Context, sys::Sys};

pub fn main<X: Sys>(ctx: Context<X>) -> i32 {
    let io = ctx.io();

    let op = io.input("op=");
    let a_str = io.input("A=");
    let b_str = io.input("B=");

    let a = a_str.parse::<i32>().unwrap_or(0);
    let b = b_str.parse::<i32>().unwrap_or(0);

    let result = match op.as_str() {
        "+" => a + b,
        "-" => a - b,
        "*" => a * b,
        "/" => a / b,
        _ => 0,
    };

    let output = format!("{} {} {} = {}", a, op, b, result);
    io.println(&output);

    0
}
