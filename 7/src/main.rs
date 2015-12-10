use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

#[derive(Clone,Debug)]
enum Scalar {
    Value(isize),
    Lookup(String)
}

#[derive(Clone,Debug)]
enum Op {
    Let(Scalar),
    And(Scalar, Scalar),
    Or(Scalar, Scalar),
    Not(Scalar),
    LShift(Scalar, isize),
    RShift(Scalar, isize),
}

#[derive(Clone,Debug)]
enum Cacheable {
    NoCache(Op),
    Cache(isize)
}

fn calc_scalar(map: &mut HashMap<String,Cacheable>, s: Scalar) -> isize {
    match s {
        Scalar::Value(v) => v,
        Scalar::Lookup(k) => calculate(map, k)
    }
}

fn calculate(map: &mut HashMap<String,Cacheable>, res: String) -> isize {
    let c_op = (*map.get(&res).unwrap()).clone();
    match c_op {
        Cacheable::Cache(v) => v,
        Cacheable::NoCache(op) => {
            let value = match op {
                Op::Let(s) => calc_scalar(map, s),
                Op::And(x, y) => calc_scalar(map, x) & calc_scalar(map, y),
                Op::Or(x, y) => calc_scalar(map, x) | calc_scalar(map, y),
                Op::Not(s) => 0xffff ^ calc_scalar(map, s),
                Op::LShift(x, y) => calc_scalar(map, x) << y,
                Op::RShift(x, y) => calc_scalar(map, x) >> y,
            };
            map.insert(res, Cacheable::Cache(value));
            value
        }
    }
}

fn parse_scalar(s: &str) -> Scalar {
    let v: Result<isize,_> = s.parse();
    match v {
        Ok(n) => Scalar::Value(n),
        Err(_) => Scalar::Lookup(s.to_string())
    }
}

fn parse_line2(line: &str) -> (Op,String) {
    let p: Vec<&str> = line.split("->").collect();
    let key = p[1].to_string().trim().to_string();

    let p0 = p[0].to_string();
    let part1: Vec<&str> = p0.trim().split(" ").collect();
    (match part1.len() {
        1 => Op::Let(parse_scalar(part1[0])),
        2 => if part1[0] == "NOT" { Op::Not(parse_scalar(part1[1])) } else { panic!("Unknown instruction: {}", line) },
        3 => match part1[1] {
            "AND" => Op::And(parse_scalar(part1[0]), parse_scalar(part1[2])),
            "OR" => Op::Or(parse_scalar(part1[0]), parse_scalar(part1[2])),
            "LSHIFT" => Op::LShift(parse_scalar(part1[0]), part1[2].parse().unwrap()),
            "RSHIFT" => Op::RShift(parse_scalar(part1[0]), part1[2].parse().unwrap()),
            _ => panic!("Unknown instruction: {}", line)
        },
        _ => panic!("Unknown instruction: {}", line)
    }, key)
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut map: HashMap<String, Cacheable> = HashMap::new();
    for line in reader.lines() {
        let l = line.unwrap();
        let (op,key) = parse_line2(&l);
        map.insert(key, Cacheable::NoCache(op));
    }
    let mut map2 = map.clone();

    let a = calculate(&mut map, "a".to_string());
    println!("a is: {}", a);

    println!("setting b to: {}", a);
    map2.insert("b".to_string(), Cacheable::NoCache(Op::Let(Scalar::Value(a))));
    let a2 = calculate(&mut map2, "a".to_string());
    println!("a is: {}", a2);
}
