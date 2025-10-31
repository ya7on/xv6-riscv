use crate::sys::Sys;
use core::str;

fn atoi(s: &[u8]) -> i32 {
    let mut result = 0;
    let mut sign = 1;

    let mut i = 0;
    if !s.is_empty() && s[0] == b'-' {
        sign = -1;
        i += 1;
    }

    while i < s.len() {
        let c = s[i];
        if c >= b'0' && c <= b'9' {
            result = result * 10 + (c - b'0') as i32;
        } else {
            break;
        }
        i += 1;
    }

    result * sign
}

pub fn u32_to_str<'a>(mut n: u32, out: &'a mut [u8]) -> &'a str {
    if n == 0 {
        out[0] = b'0';
        return unsafe { str::from_utf8_unchecked(&out[..1]) };
    }

    let mut i = 0;
    while n > 0 {
        let d = (n % 10) as u8;
        out[i] = b'0' + d;
        n /= 10;
        i += 1;
    }

    out[..i].reverse();

    unsafe { str::from_utf8_unchecked(&out[..i]) }
}

pub fn main<X: Sys>(x: &X) -> i32 {
    let mut a_buf = [0u8; 64];
    let mut b_buf = [0u8; 64];
    let mut op_buf = [0u8; 64];

    let a_len_u32 = x.readln("A=", &mut a_buf);
    let b_len_u32 = x.readln("B=", &mut b_buf);
    let op_len_u32 = x.readln("op=", &mut op_buf);

    let a_len = core::cmp::min(a_len_u32 as usize, a_buf.len());
    let b_len = core::cmp::min(b_len_u32 as usize, b_buf.len());
    let op_len = core::cmp::min(op_len_u32 as usize, op_buf.len());

    let a = atoi(&a_buf[..a_len]);
    let b = atoi(&b_buf[..b_len]);
    let op = &op_buf[..op_len];

    let mut num_buf = [0u8; 12];
    let s = u32_to_str(a as u32, &mut num_buf);
    x.write(s);
    x.write(" ");
    unsafe {
        x.write(str::from_utf8_unchecked(op));
    };
    x.write(" ");
    let mut num_buf = [0u8; 12];
    let s = u32_to_str(b as u32, &mut num_buf);
    x.write(s);
    x.write(" = ");

    let opch = if op_len > 0 { op_buf[0] } else { 0 };

    let result = if opch == b'+' {
        a + b
    } else if opch == b'-' {
        a - b
    } else if opch == b'*' {
        a * b
    } else {
        0
    };

    let mut num_buf = [0u8; 12];
    let s = u32_to_str(result as u32, &mut num_buf);
    x.writeln(s);

    0
}
