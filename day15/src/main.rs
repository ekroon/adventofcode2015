use scan_fmt::scan_fmt;
use std::io::{stdin, BufRead};
use std::ops::Mul;

const SCAN_LINE: &str = "{}: capacity {}, durability {}, flavor {}, texture {}, calories {}";

fn main() {
    solve();
}

fn solve() {
    let stdin = stdin();
    let handle = stdin.lock();
    let parsed_lines = handle
        .lines()
        .map(|l| scan_fmt!(&l.unwrap(), SCAN_LINE, String, i32, i32, i32, i32, i32).unwrap())
        .collect::<Vec<_>>();
    // println!("{:?}", parsed_lines);
    let mut max_score = 0;
    let mut max_500_score = 0;
    let items = parsed_lines.as_slice();
    for i in 0..100 {
        for j in 0..(100 - i) {
            for k in 0..(100 - i - j) {
                let l = 100 - i - j - k;
                let score1 = i * items[0].1 + j * items[1].1 + k * items[2].1 + l * items[3].1;
                let score2 = i * items[0].2 + j * items[1].2 + k * items[2].2 + l * items[3].2;
                let score3 = i * items[0].3 + j * items[1].3 + k * items[2].3 + l * items[3].3;
                let score4 = i * items[0].4 + j * items[1].4 + k * items[2].4 + l * items[3].4;
                let score5 = i * items[0].5 + j * items[1].5 + k * items[2].5 + l * items[3].5;

                let scores = [score1, score2, score3, score4];
                let score = if scores.iter().any(|&s| s <= 0) {
                    0
                } else {
                    scores.iter().fold(1, Mul::mul)
                };
                if score > max_score {
                    max_score = score;
                }
                if score5 == 500 && score > max_500_score {
                    max_500_score = score;
                }
            }
        }
    }
    println!("part 1: {}", max_score);
    println!("part 2: {}", max_500_score);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scan_line_test() {
        let line = "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8";
        let scanned = scan_fmt!(line, SCAN_LINE, String, i32, i32, i32, i32, i32);
        assert_eq!(scanned, Ok(("Butterscotch".to_string(), -1, -2, 6, 3, 8)));
    }
}
