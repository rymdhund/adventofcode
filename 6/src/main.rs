use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

#[derive(Debug)]
enum Command {
    On,
    Off,
    Toggle
}

#[derive(Debug)]
struct Instruction {
    command: Command,
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize

}

fn light(instructions: &[Instruction]) -> usize {
    let mut lights = [[0u8; 1000]; 1000];

    for i in instructions {
        for x in i.x1..(i.x2+1) {
            for y in i.y1..(i.y2+1) {
                match i.command {
                    Command::On => lights[x][y] = 1,
                    Command::Off => lights[x][y] = 0,
                    Command::Toggle => lights[x][y] ^= 1
                }
            }
        }
    }

    let mut cnt = 0;
    for x in 0..1000 {
        for y in 0..1000 {
            if lights[x][y] == 1 {
                cnt += 1;
            } else if lights[x][y] != 0 {
                panic!("non 1 or 0 light!");
            }
        }
    }
    cnt
}

#[test]
fn test_light() {
    assert_eq!(1000000, light(&[Instruction {command: Command::On, x1: 0, y1: 0, x2: 999, y2: 999 }] ));
    assert_eq!(1000, light(&[Instruction {command: Command::Toggle, x1: 0, y1: 0, x2: 999, y2: 0 }] ));
    assert_eq!(1000, light(&[Instruction {command: Command::Toggle, x1: 0, y1: 0, x2: 0, y2: 999 }] ));
    assert_eq!(0, light(&[Instruction {command: Command::Toggle, x1: 0, y1: 0, x2: 999, y2: 0 },
                        Instruction {command: Command::Toggle, x1: 0, y1: 0, x2: 999, y2: 0 }
    ] ));
    assert_eq!(1000000-4, light(&[Instruction {command: Command::On, x1: 0, y1: 0, x2: 999, y2: 999 },
                                Instruction {command: Command::Off, x1: 499, y1: 499, x2: 500, y2: 500 }] ));
    assert_eq!(1, light(&[Instruction {command: Command::Toggle, x1: 7, y1: 7, x2: 7, y2: 7 }] ));
}

fn light2(instructions: &[Instruction]) -> usize {
    let mut lights = [[0u8; 1000]; 1000];

    for i in instructions {
        for x in i.x1..(i.x2+1) {
            for y in i.y1..(i.y2+1) {
                match i.command {
                    Command::On => lights[x][y] += 1,
                    Command::Off => lights[x][y] = if lights[x][y] > 0 { lights[x][y] - 1 } else { 0 },
                    Command::Toggle => lights[x][y] += 2
                }
            }
        }
    }

    let mut cnt = 0;
    for x in 0..1000 {
        for y in 0..1000 {
            cnt += lights[x][y] as usize;
        }
    }
    cnt
}

fn parse_coords(s: &str) -> (usize, usize, usize, usize) {
    let words: Vec<&str> = s.split(" ").collect();
    let v1 = words[0].split(",").collect::<Vec<&str>>();
    let v2 = words[2].split(",").collect::<Vec<&str>>();
    (v1[0].parse().unwrap(), v1[1].parse().unwrap(), v2[0].parse().unwrap(), v2[1].parse().unwrap())
}

fn parse(line: &str) -> Instruction {
    if line.starts_with("turn on") {
        let (x1, y1, x2, y2) = parse_coords(&line[8..]);
        Instruction { command: Command::On, x1: x1, y1: y1, x2: x2, y2: y2 }
    } else if line.starts_with("turn off") {
        let (x1, y1, x2, y2) = parse_coords(&line[9..]);
        Instruction { command: Command::Off, x1: x1, y1: y1, x2: x2, y2: y2 }
    } else if line.starts_with("toggle") {
        let (x1, y1, x2, y2) = parse_coords(&line[7..]);
        Instruction { command: Command::Toggle, x1: x1, y1: y1, x2: x2, y2: y2 }
    } else {
        panic!("Unknown instruction: {}", line)
    }
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut instructions: Vec<Instruction> = Vec::new();
    for line in reader.lines() {
        let l = line.unwrap();
        let i = parse(&l);
        instructions.push(i);
    }
    println!("lights: {}", light(&instructions));
    println!("lights2: {}", light2(&instructions));
}
