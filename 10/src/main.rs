use std::env;

// the longer the line the better!
fn look_and_say(s: &str) -> String {
    let (c, n, sum) = s.to_string().chars().fold((s.chars().next().unwrap(), 0, "".to_string()), |(c, n, sum), x| if x == c { (c, n+1, sum) } else { (x, 1, sum + &n.to_string() + &c.to_string()) });
    sum + &n.to_string() + &c.to_string()
}

fn main() {
    let mut s = env::args().skip(1).next().expect("No base string provided");

    for _ in 0..40 {
        s = look_and_say(&s);
    }
    println!("{}", s.len());
    for _ in 0..10 {
        s = look_and_say(&s);
    }
    println!("{}", s.len());
}
