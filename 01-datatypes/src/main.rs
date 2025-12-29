use std::{f32, f64, i8, i16, i32, u8, u16, u32};

fn main() {
    // &str
    // literal string
    let _str: &str = "Hello, World!";

    // String
    // growable string
    let _string: String = String::from("Hello, World!");

    // integer
    // i : signed integer
    // u : unsigned integer
    let _i8: i8 = i8::MAX;
    let _i16: i16 = i16::MAX;
    let _i32: i32 = i32::MAX;

    let _u8: u8 = u8::MAX;
    let _u16: u16 = u16::MAX;
    let _u32: u32 = u32::MAX;

    let _f32: f32 = f32::MAX;
    let _f64: f64 = f64::MAX;

    // char
    // for save a character
    // only with ''
    let _ch: char = 'A';
    let _emoji: char = '🦀';
    let _number: char = '1';
    let _ch2: char = 'ℤ';

    // tuple
    let _tuple: (i32, String, char) = (42, String::from("Hello, World!"), 'ℤ');
    let (_x, _y, _z) = _tuple;
    let _n: i32 = _tuple.0;

    // array
    let _array: [i32; 5] = [1, 2, 3, 4, 5];
    let _array2: [i32; 10] = [5; 10];

    let _five: i32 = _array2[0];

    // parse data
    let _my_string: String = String::from("12");
    let _my_integer: i32 = _my_string
        .trim()
        .parse()
        .expect("the string is not a number");
}
