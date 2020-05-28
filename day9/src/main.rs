use std::collections::{HashMap, HashSet};
use std::io::{stdin, BufRead};

use itertools::Itertools;

fn parse_line(line: &str) -> (&str, &str, u32) {
    let mut parts = line.split_whitespace();
    let from = parts.next().unwrap();
    parts.next();
    let to = parts.next().unwrap();
    parts.next();
    let distance = parts.next().unwrap().parse::<u32>().unwrap();
    (from, to, distance)
}

fn solve() {
    let stdin = stdin();
    let handle = stdin.lock();
    let lines: Vec<String> = handle.lines().map(Result::unwrap).collect();
    let routes = lines.iter().map(|s| parse_line(s)).collect::<Vec<_>>();
    let distances = routes
        .iter()
        .fold(HashMap::<(&str, &str), u32>::new(), |mut acc, r| {
            acc.insert((r.0, r.1), r.2);
            acc.insert((r.1, r.0), r.2);
            acc
        });
    let places = routes
        .iter()
        .flat_map(|r| vec![r.0, r.1])
        .collect::<HashSet<_>>();
    let total_places = (&places).len();
    let all_routes = places.into_iter().permutations(total_places);
    let route_distances = all_routes
        .map(|route| {
            let r = route.clone().into_iter();
            let mut r_drop = route.into_iter();
            r_drop.next();
            let combined = r.zip(r_drop);
            combined.map(|s| distances.get(&s).unwrap()).sum::<u32>()
        })
        .collect::<Vec<_>>();
    println!("Part 1: {}", route_distances.iter().min().unwrap(),);
    println!("Part 2: {}", route_distances.iter().max().unwrap(),);
}

fn main() {
    solve();
}
