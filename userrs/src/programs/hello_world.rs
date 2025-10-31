use alloc::string::String;

use crate::sys::Sys;

pub fn main<X: Sys>(x: &X) -> i32 {
    let mut s = String::new();
    s.push_str("Hello, World from H_E_A_P!");
    x.writeln(s.as_str());
    0
}
