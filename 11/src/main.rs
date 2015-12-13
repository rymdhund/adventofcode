use std::env;

fn inc(s: &mut [u8]) {
    for i in (0..s.len()).rev() {
        s[i] += 1;
        if s[i] > 'z' as u8 {
            s[i] = 'a' as u8;
        } else if s[i] == 'i' as u8 || s[i] == 'o' as u8 || s[i] == 'l' as u8 {
            // optimization that skips ranges of invalid strings
            s[i] += 1;
            for j in (i+1)..s.len() {
                s[j] = 'a' as u8;
            }
            return;
        } else {
            return;
        }
    }
}

fn next_pw(s: &str) -> String {
    let mut next = s.as_bytes().to_owned();
    loop {
        inc(&mut *next);
        let straight = next.windows(3).any(|x| x[0]+1 == x[1] && x[1]+1 == x[2]);
        let pairs = next
            .windows(2)
            .filter(|x| x[0] == x[1])
            .map(|x| x[0])
            .fold(vec![], |mut unique, x| if unique.contains(&x) { unique } else { unique.push(x); unique })
            .len() >= 2;

        if straight && pairs {
            return String::from_utf8(next).unwrap();
        }
    }
}

#[test]
fn test_next_pw() {
    assert_eq!("abcdffaa", next_pw("abcdefgh"));
}

fn main() {
    let input = env::args().skip(1).next().expect("No starting password provided");
    let next = next_pw(&input);
    println!("{}", next);
    println!("{}", next_pw(&next));
}
