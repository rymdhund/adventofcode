extern crate regex;

use regex::Regex;

fn decoded_size(string: &str) -> usize {
    let one = Regex::new(r#"(\\")|(\\\\)"#).unwrap();
    let three = Regex::new(r#"(\\x[\da-f]{2})"#).unwrap();
    string.len() - one.captures_iter(string).count() - three.captures_iter(string).count()*3 - 2
}

#[test]
fn test_decoded_size() {
    assert_eq!(7, decoded_size(r#""aaa\"aaa""#));
    assert_eq!(1, decoded_size(r#""\x27""#));
}

fn encoded_size(string: &str) -> usize {
    let chars = Regex::new(r#"(\\)|(")"#).unwrap();
    string.len() + chars.captures_iter(string).count() + 2
}

fn part1(input: &str) -> usize {
    input.lines().map(|line| line.len() - decoded_size(line)).fold(0, |sum,x| sum+x)
}

fn part2(input: &str) -> usize {
    input.lines().map(|line| encoded_size(line) - line.len()).fold(0, |sum,x| sum+x)
}

fn main() {
    let input = include_str!("../input.txt");
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
}
