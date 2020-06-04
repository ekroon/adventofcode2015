use itertools::Itertools;
use std::io::{stdin, BufRead};
use std::ops::Deref;

fn main() {
    solve();
}

fn solve() {
    let stdin = stdin();
    let handle = stdin.lock();
    let buckets = handle
        .lines()
        .map(|l| l.unwrap().parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let mut valid_combinations = 0;
    let mut minimum_valid_combinations = 0;
    for i in 1..=(&buckets).len() {
        valid_combinations += buckets
            .iter()
            .combinations(i)
            .filter(|v| 150 == v.iter().map(Deref::deref).sum::<i32>())
            .count();
        if minimum_valid_combinations == 0 {
            minimum_valid_combinations = valid_combinations;
        }
    }
    println!("{}", valid_combinations);
    println!("{}", minimum_valid_combinations);
}
