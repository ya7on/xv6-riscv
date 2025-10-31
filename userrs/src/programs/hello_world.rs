use crate::user::{UserAPI, Xv6};

pub fn main<X: UserAPI>() -> i32 {
    Xv6::writeln("Hello, World!".into());
    0
}
