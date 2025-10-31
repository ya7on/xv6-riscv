use alloc::string::String;

use crate::user::{UserAPI, Xv6};

pub fn main<X: UserAPI>() -> i32 {
    let mut s = String::new();
    s.push_str("Hello, World from H_E_A_P!");
    Xv6::writeln(s.as_str());
    0
}
