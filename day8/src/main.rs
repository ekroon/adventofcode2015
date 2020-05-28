use std::io::{stdin, BufRead};

fn solve() {
    let stdin = stdin();
    let handle = stdin.lock();
    let lines = handle.lines().map(|r| r.unwrap()).collect::<Vec<_>>();
    let chars = (&lines)
        .iter()
        .map(|s| s.chars())
        .flatten()
        .collect::<Vec<char>>();
    let total_chars = chars.len();
    let mut parsed_chars = 0usize;
    let mut generated_chars = 0usize;
    let mut char_iter = chars.iter();
    while let Some(c) = char_iter.next() {
        if c == &'"' {
            generated_chars += 3;
        } else if c != &'\\' {
            parsed_chars += 1;
            generated_chars += 1;
        } else {
            let next_char = char_iter.next().unwrap();
            if next_char == &'\\' || next_char == &'"' {
                generated_chars += 2;
            } else if next_char == &'x' {
                char_iter.next();
                char_iter.next();
                generated_chars += 3;
            }
            parsed_chars += 1;
            generated_chars += 2;
        }
    }
    println!("part 1: {}", total_chars - parsed_chars);
    println!("part 2: {}", generated_chars - total_chars);
}

fn main() {
    solve();
}
