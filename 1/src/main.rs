use std::fs::File;
use std::io::Read;

fn level(input: &str) -> isize {
    let mut cnt = 0;
    for c in input.chars() {
        cnt += match c {
            '(' => 1,
            ')' => -1,
            _   => 0,
        }
    }
    cnt
}

#[test]
fn test_level() {
    assert_eq!(0, level("(())"));
    assert_eq!(0, level("()()"));
    assert_eq!(3, level("))((((("));
}

fn basement(input: &str) -> Result<usize, String>{
    let mut cnt = 0;
    for (i,c) in input.chars().enumerate() {
        cnt += match c {
            '(' => 1,
            ')' => -1,
            _   => 0
        };
        if cnt < 0 { return Ok(i+1); }
    }
    Err("Santa never enters the basement".to_string())
}

#[test]
fn test_basement() {
    assert_eq!(1, basement(")").unwrap());
    assert_eq!(5, basement("()())").unwrap());
}

fn main() {
    let mut file = File::open("input.txt").ok().expect("No such file: input.txt");
    let mut input = String::new();
    file.read_to_string(&mut input).ok().expect("Couldn't read file");

    println!("{}",level(&input));
    println!("{}",basement(&input).unwrap());
}
