use std::collections::HashSet;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

#[derive(Clone)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}

fn mv(pos: (isize, isize), m: Move) -> (isize, isize) {
    let (x,y) = pos;
    match m {
        Move::Up => (x, y+1),
        Move::Down => (x, y-1),
        Move::Left => (x-1, y),
        Move::Right => (x+1, y)
    }
}

fn houses(moves: &[Move]) -> usize {
    let mut pos = (0,0);
    let mut been: HashSet<(isize, isize)> = HashSet::new();
    been.insert(pos.clone());

    for m in moves.iter() {
        pos = mv(pos, m.clone());
        been.insert(pos.clone());
    }

    been.len()
}

fn houses2(moves: &[Move]) -> usize {
    let mut pos = (0,0);
    let mut been: HashSet<(isize, isize)> = HashSet::new();
    been.insert(pos.clone());

    for (_,m) in moves.iter().enumerate().filter(|&(i,_)| i % 2 == 0) {
        pos = mv(pos, m.clone());
        been.insert(pos.clone());
    }

    pos = (0,0);
    for (_,m) in moves.iter().enumerate().filter(|&(i,_)| i % 2 == 1) {
        pos = mv(pos, m.clone());
        been.insert(pos.clone());
    }

    been.len()
}

#[test]
fn test_houses() {
    assert_eq!(2, houses(&[Move::Right]));
}

fn parse(line: &str) -> Vec<Move> {
    line.chars().filter_map(|x| match x {
        '^' => Some(Move::Up),
        '>' => Some(Move::Right),
        '<' => Some(Move::Left),
        'v' => Some(Move::Down),
        _   => None
    }).collect()
}

#[test]
fn test_parse_and_houses() {
    assert_eq!(2, houses(&parse(">")[..]));
    assert_eq!(4, houses(&parse("^>v<")[..]));
    assert_eq!(2, houses(&parse("^v^v^v^v^v")[..]));
}

#[test]
fn test_parse_and_houses2() {
    assert_eq!(3, houses2(&parse("^v")[..]));
    assert_eq!(3, houses2(&parse("^>v<")[..]));
    assert_eq!(11, houses2(&parse("^v^v^v^v^v")[..]));
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let mut reader = BufReader::new(file).ok().expect("Couldn't read file");
    let mut buffer = String::new();

    reader.read_line(&mut buffer);
    println!("houses: {}", houses(&parse(&buffer[..])));
    println!("houses2: {}", houses2(&parse(&buffer[..])));
}
