use std::collections::HashMap;
use std::collections::HashSet;

fn h<'a>(happiness: &HashMap<(&'a str,&'a str), isize>, a: &'a str, b: &'a str) -> isize {
    happiness.get(&(a,b)).unwrap_or(&0) + happiness.get(&(b,a)).unwrap_or(&0)
}

fn max1<'a>(happiness: &HashMap<(&'a str,&'a str), isize>, people_left: &HashSet<&'a str>, first: &'a str, last: &'a str) -> isize {
    if people_left.len() == 1 {
        let &p = people_left.iter().next().unwrap();
        return h(happiness, p, first) + h(happiness, p, last);
    }
    let mut mx = std::isize::MIN;

    for &p in people_left.iter() {
        let mut pl = people_left.clone();
        pl.remove(p);
        let v = h(happiness, p, last) + max1(happiness, &pl, first, p);
        if v > mx {
            mx = v;
        }
    }
    mx
}

fn max<'a>(happiness: &HashMap<(&'a str,&'a str), isize>, people: &HashSet<&'a str>) -> isize {
    let mut pl = people.clone();
    let first = people.iter().next().unwrap(); // we can pick this arbitrarily since it's a round table
    pl.remove(first);
    max1(happiness, &pl, first, first)
}

fn parse(input: &str) -> (HashMap<(&str,&str),isize>, HashSet<&str>) {
    let mut h: HashMap<(&str,&str), isize> = HashMap::new();
    let mut p = HashSet::new();

    for line in input.lines() {
        let x: Vec<&str> = line.split(" would ").collect();
        let name1 = x[0];
        let y: Vec<&str> = x[1].split(" happiness units by sitting next to ").collect();
        let name2 = y[1].trim_right_matches(".");
        let v = if y[0].starts_with("lose") {
            - y[0][5..].parse::<isize>().unwrap()
        } else {
            y[0][5..].parse().unwrap()
        };
        h.insert((name1, name2), v);
        p.insert(name1);
        p.insert(name2);
    }
    (h, p)
}

fn main() {
    let input = include_str!("../input.txt");
    let (h, p) = parse(input);
    println!("max happiness: {}", max(&h, &p));

    // if i had constructed a seating list i could do this faster
    let mut p2 = p.clone();
    p2.insert("Myself");
    println!("with myself: {}", max(&h, &p2));
}
