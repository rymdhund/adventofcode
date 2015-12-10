extern crate regex;

use regex::Regex;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

fn double(s: &str) -> bool {
    let mut last = '_';

    for c in s.chars() {
        if c == last {
            return true;
        }
        last = c;
    }
    false
}

fn nice(s: &str) -> bool {
    let wovels = Regex::new(r"[aeiou].*[aeiou].*[aeiou]").unwrap();
    let forbidden = Regex::new(r"ab|cd|pq|xy").unwrap();

    wovels.is_match(&s) && double(&s) && !forbidden.is_match(&s)
}

#[test]
fn test_nice() {
    assert_eq!(true, nice("ugknbfddgicrmopn"));
    assert_eq!(true, nice("aaa"));
    assert_eq!(false, nice("jchzalrnumimnmhp"));
    assert_eq!(false, nice("haegwjzuvuyypxyu"));
    assert_eq!(false, nice("dvszwmarrgswjxmb"));
}

fn twopair(s: &str) -> bool {
    if s.len() < 2 {
        return false
    }
    for i in 0..(s.len()-1) {
        if s[(i+2)..].contains(&s[i..(i+2)]) {
            return true;
        }
    }
    false
}

#[test]
fn test_twopair() {
    assert_eq!(false, twopair("ieodomkazucvgmuy"));
    assert_eq!(true, twopair("qjhvhtzxzqqjkmpb"));
}

fn triple(s: &str) -> bool {
    if s.len() < 3 {
        return false
    }
    for i in 1..(s.len()-2) {
        // note: this only works for ascii
        if s.as_bytes()[i] == s.as_bytes()[i+2] {
            return true;
        }
    }
    false
}

fn nice2(s: &str) -> bool {
    twopair(s) && triple(s)
}

#[test]
fn test_nice2() {
    assert_eq!(true, nice2("qjhvhtzxzqqjkmpb"));
    assert_eq!(true, nice2("xxyxx"));
    assert_eq!(false, nice2("a"));
    assert_eq!(false, nice2("ab"));
    assert_eq!(false, nice2("abc"));
    assert_eq!(false, nice2("uurcxstgmygtbstg"));
    assert_eq!(false, nice2("ieodomkazucvgmuy"));
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut cnt1 = 0;
    let mut cnt2 = 0;
    for line in reader.lines() {
        let l = line.unwrap();
        if nice(&l) {
            cnt1 += 1;
        }
        if nice2(&l) {
            cnt2 += 1;
        }
    }
    println!("nice words: {}", cnt1);
    println!("nice words2: {}", cnt2);
}
