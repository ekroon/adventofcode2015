use itertools::Itertools;
use std::char;

fn expand(mut number_string: String) -> String {
    let chars = number_string.chars().collect_vec();
    number_string.clear();
    for (key, group) in &chars.into_iter().group_by(|c| *c) {
        let count = group.count() as u32;
        number_string.push(char::from_digit(count, 10).unwrap());
        number_string.push(key);
    }
    number_string
}

fn solve() {
    let mut s = "1113122113".to_string();
    for _ in 0..40 {
        s = expand(s);
    }
    println!("part 1: {}", s.len());
    for _ in 0..10 {
        s = expand(s);
    }
    println!("part 2: {}", s.len());
}

fn main() {
    solve();
}
