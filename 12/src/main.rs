extern crate rustc_serialize;

use rustc_serialize::json::Json;
use std::fs::File;
use std::io::Read;

fn number_sum1(json: &Json) -> i64 {
    match json {
        &Json::I64(n) => n,
        &Json::U64(n) => n as i64,
        &Json::F64(n) => n as i64,
        &Json::Array(ref a) => a.iter().fold(0, |sum,x| sum + number_sum1(x)),
        &Json::Object(ref o) => o.values().fold(0, |sum,x| sum + number_sum1(x)),
        _ => 0
    }
}

fn number_sum2(json: &Json) -> i64 {
    match json {
        &Json::I64(n) => n,
        &Json::U64(n) => n as i64,
        &Json::F64(n) => n as i64,
        &Json::Array(ref a) => a.iter().fold(0, |sum,x| sum + number_sum2(x)),
        &Json::Object(ref o) => {
            if o.values().any(|x| x.as_string().map_or(false, |s| s == "red")) {
                0
            } else {
                o.values().fold(0, |sum,x| sum + number_sum2(x))
            }
        },
        _ => 0
    }
}

fn main() {
    let mut file = File::open("input.json").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let json = Json::from_str(&data).unwrap();
    println!("sum part1: {}", number_sum1(&json));
    println!("sum part2: {}", number_sum2(&json));
}
