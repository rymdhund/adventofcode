use std::cmp;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

fn paper(packet: (usize,usize,usize)) -> usize {
    let (w,l,h) = packet;
    let a = w*l;
    let b = w*h;
    let c = l*h;
    2*a + 2*b + 2*c + cmp::min(cmp::min(a, b), c)
}

#[test]
fn test_paper() {
    assert_eq!(58, paper((2,3,4)));
    assert_eq!(58, paper((3,2,4)));
    assert_eq!(58, paper((4,3,2)));
    assert_eq!(43, paper((1,1,10)));
}

fn ribbon(packet: (usize, usize, usize)) -> usize {
    let (w, l, h) = packet;
    let a = 2*(w+l);
    let b = 2*(w+h);
    let c = 2*(l+h);
    cmp::min(a, cmp::min(b, c)) + w*l*h
}

#[test]
fn test_ribbon() {
    assert_eq!(34, ribbon((2,3,4)));
    assert_eq!(14, ribbon((1,1,10)));
}

fn parse_file(filename: &str) -> Vec<(usize, usize, usize)> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    fn parse_line(line: String) -> (usize, usize, usize) {
        let nums: Vec<usize> = line.split("x").map(|x| x.parse::<usize>().unwrap()).collect();
        if nums.len() == 3 {
            (nums[0], nums[1], nums[2])
        } else {
            panic!(format!("Couldn't parse line: {}", line))
        }
    }

    reader.lines().filter_map(|result| result.ok().map(|line| parse_line(line))).collect()
}

fn main() {
    let sizes = parse_file("input.txt");
    println!("paper: {}", sizes.iter().map(|&x| paper(x)).fold(0, |sum, x| sum + x));
    println!("ribbon: {}", sizes.iter().map(|&x| ribbon(x)).fold(0, |sum, x| sum + x));
}
