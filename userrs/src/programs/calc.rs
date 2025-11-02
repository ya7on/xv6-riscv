use alloc::format;

use crate::{context::Context, sys::Sys};

pub fn main<X: Sys>(ctx: Context<X>) -> i32 {
    let io = ctx.io();

    let op = io.input("op=").expect("Failed to read operation");
    let a_str = io.input("A=").expect("Failed to read A");
    let b_str = io.input("B=").expect("Failed to read B");

    let a = a_str.parse::<i32>().expect("Failed to parse A");
    let b = b_str.parse::<i32>().expect("Failed to parse B");

    let result = match op.as_str() {
        "+" => a + b,
        "-" => a - b,
        "*" => a * b,
        "/" => a / b,
        _ => panic!("Invalid operation"),
    };

    let output = format!("{} {} {} = {}", a, op, b, result);
    io.println(&output);

    0
}
