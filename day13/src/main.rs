use std::collections::{HashMap, HashSet};
use std::io::{stdin, BufRead};

use itertools::Itertools;

fn main() {
    solve();
}

fn solve() {
    let stdin = stdin();
    let handle = stdin.lock();
    let lines = handle.lines().map(|l| l.unwrap()).collect::<Vec<_>>();
    let parsed_lines: Vec<(&str, &str, i32)> = lines
        .iter()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let from = parts.next().unwrap();
            parts.next();
            let negative = parts.next().unwrap() == "lose";
            let mut units = parts.next().unwrap().parse::<i32>().unwrap();
            if negative {
                units = -units;
            }
            let mut to = parts.dropping(6).next().unwrap();
            to = to.split('.').next().unwrap();
            (from, to, units)
        })
        .collect();
    let mut people = parsed_lines.iter().map(|l| l.0).collect::<HashSet<_>>();
    for (i, extra) in vec![vec![], vec!["Me"]].iter().enumerate() {
        for p in extra {
            (&mut people).insert(p);
        }
        let total_people = (&people).len();
        let all_orders = (&people)
            .iter()
            .permutations(total_people)
            .collect::<Vec<_>>();
        let happiness_pairs: HashMap<(&str, &str), i32> =
            parsed_lines.iter().map(|l| ((l.0, l.1), l.2)).collect();
        let scores = all_orders
            .iter()
            .map(|o| {
                let mut v = o.clone();
                v.push(o.get(0).unwrap());
                let mut total = 0;
                for s in v.as_slice().windows(2) {
                    total += *happiness_pairs.get(&(*s[0], *s[1])).or(Some(&0)).unwrap();
                    total += *happiness_pairs.get(&(*s[1], *s[0])).or(Some(&0)).unwrap();
                }
                total
            })
            .collect::<Vec<_>>();
        println!("Part {}: {:?}", i + 1, scores.iter().max().unwrap());
    }
}
