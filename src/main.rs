fn main() {
    println!("Hello, world!");
}

// Mandelbrot set
// The Mandelbrot set is defined as the set of complex numbers c for which z does not fly out to infinity.

// Infinite loop, Squaring any number smaller than 1 makes it smaller, so it approaches zero; squaring 1 yields 1; squaring a number larger than 1 makes it larger, so it approaches infinity; and squaring a negative number makes it positive, after which it behaves as one of the prior cases
// fn square_loop(mut x: f64) {
//     loop {
//         x = x * x;
//     }
// }

// Modified
// If c is greater than 0.25, or less than –2.0, then x eventually becomes infinitely large; otherwise, it stays somewhere in the neighborhood of zero.
// fn square_add_loop(c: f64) {
//     let mut x = 0;
//     loop {
//         x = x * x + c;
//     }
// }

// Using the num crate on crates.io allows us to use complex number types. We import it into Cargo.toml

// Updated version using Complex numbers
extern crate num;
use num::Complex;

#[allow(dead_code)]
fn complex_square_add_loop(c: Complex<f64>) {
    // re: 0.0... is the way we write complex zero using the num crate's Complex type. Complex is a Rust structure type (or struct), defined like this:
    // struct Complex<T> {
        // Real portion of the complex number
        // re: T,

        // Imaginary portion of the complex number
        // im: T
    // }
    // The preceding code defines a struct name Complex, with two fields, re and im. Complex is a generic structure. We can read the <T> as "for any type T". The Complex value for re and im a f64 values as we declared it in the function definition up top.
    let mut z = Complex { re: 0.0, im: 0.0};
    loop {
        // Using 'z' is traditional for complex numbers
        // The num create arranges for *, + and other operators to work on Complex values, it allows the function to operate on the complex plane, not just along the real number line.
        z = z * z + c;
    }
}

// FInal Version

// Comments with /// as per below are documentation comments which rustdoc utility knows how to parse.
/// Try to determine if `c` is in the Mandelbrot set, using at most `limit`
/// iterations to decide.
///
/// If `c` is not a member, return `Some(i)`, where `i` is the number of
/// iterations it took for `c` to leave the circle of radius two centered on the
/// origin. If `c` seems to be a member (more precisely, if we reached the
/// iteration limit without being able to prove that `c` is not a member),
/// return `None`.

// This function takes the complex number c that we want to test for membership in the Mandelbrot set, and a limit on the number of iterations to try before giving up and declaring c to probably be a member.
// The function;s return value is an Option<u32>. An Option is an enumerated type, often called an enum because its definition enumerates several variants that a value of this type could be:
// For any type T, a value of type Option<T> is either Some(v), where v is a value of type T, or None, indicating no T value is available.
// Option is a generic type, we can use it to represent an optional value of any type T we'd like.
// enum Option<T> {
//     None,
//     Some(T)
// }
fn escape_time(c: Complex<f64>, limit: u32) -> Option<u32> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    // This for loop iterates over the range of integers starting with 0 and up to (but not including) limit.
    for i in 0..limit {
        z = z * z + c;
        // The z.norm... method call returns the square of z's distance from the origin. To decide whether z has left the circle of radius two, instead of computing a square root, we just compare the squared distance with 4.0 which is faster.
        if z.norm_sqr() > 4.0 {
            // If c is not in the set, escape_time returns Some(i), where i is the number of the iteration at which z left the circle of radius two.
            return Some(i);
        }
    }
    // Otherwise, c is apparently in the set, and escape_time returns None.
    None
}

// Parsing Pair Command-Line Arguments

// The program needs several command-line arguments controlling the resolution of the image we'll write, and the portion of the Mandelbrot set the image shows.

use std::str::FromStr;

/// Parse the string `s` as a coordinate pair, like `"400x600"` or `"1.0,0.5"`.
///
/// Specifically, `s` should have the form <left><sep><right>, where <sep> is
/// the character given by the `separator` argument, and <left> and <right> are both
/// strings that can be parsed by `T::from_str`.
///
/// If `s` has the proper form, return `Some<(x, y)>`. If it doesn't parse
/// correctly, return `None`.


// parse_pair is a generic function. <T: FromStr> can be read as “For any type T that implements the FromStr trait...” It lets us define an entire family of functions at once. parse_pair::<i32> is a function that parses pairs of i32 values; parse_pair::<f64> parses pairs of floating-point values; and so on.
// Our return type is Option<(T, T)>: either None, or a value Some((v1, v2)), where (v1, v2) is a tuple of two values, both of type T.
fn parse_pair<T: FromStr>(s: &str, separator:char) -> Option<(T, T)> {
    // The function doesn't use an explicit return statement, sot its return value is the value of the last (and the only) expression in its body.
    // The find method searches the string for a character that matches separator. If find returns None, meaning that the separator character doesn't occur in the string, the entire match expression evaluates to None, the parse failed. Otherwise we take index to be the separator's position in the string.
    match s.find(separator) {
        None => None,
        Some(index) => {
            match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
                (Ok(l), Ok(r)) => Some((l, r)),
                _ => None
            }
        }
    }
}

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("", ','), None);
    assert_eq!(parse_pair::<i32>("10,", ','), None);
    assert_eq!(parse_pair::<i32>(",10", ','), None);
    assert_eq!(parse_pair::<i32>("10,20", ','), Some((10, 20)));
    assert_eq!(parse_pair::<i32>("10,20xy", ','), None);
    assert_eq!(parse_pair::<f64>("0.5x", 'x'), None);
    assert_eq!(parse_pair::<f64>("0.5x1.5", 'x'), Some((0.5, 1.5)));
}