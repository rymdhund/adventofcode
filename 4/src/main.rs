extern crate crypto;

use crypto::md5::Md5;
use crypto::digest::Digest;
use std::iter;
use std::env;

fn find_md5(start: &str, zeroes: usize) -> usize {
    let mut sh = Md5::new();

    let mut x = 1;
    let prefix: String = iter::repeat("0").take(zeroes).collect();

    loop {
        let input = start.to_string() + &x.to_string();
        sh.input_str(&input);
        let res = sh.result_str();
        if res.starts_with(&prefix) {
            return x;
        }
        sh.reset();
        x += 1;
    }
}

#[test]
fn test_find_md5() {
    assert_eq!(609043, find_md5("abcdef", 5));
}

fn main() {
    let base = env::args().skip(1).next().expect("No base string provided");
    println!("{}", find_md5(&base, 5));
    println!("{}", find_md5(&base, 6));
}
