use std::collections::{HashMap, HashSet};
use std::io::{stdin, BufRead};

use itertools::{Itertools, MinMaxResult};

fn parse_line(line: &str) -> (&str, &str, u32) {
    let mut parts = line.split_whitespace();
    let from = parts.next().unwrap();
    parts.next();
    let to = parts.next().unwrap();
    parts.next();
    let distance = parts.next().unwrap().parse::<u32>().unwrap();
    (from, to, distance)
}

fn second<A: Clone, B: Copy>(input: &(A, B)) -> B {
    input.1
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
    let route_distances = all_routes.map(|route| {
        let combined = route.as_slice().windows(2);
        let route_distance = combined
            .map(|s| distances.get(&(s[0], s[1])).unwrap())
            .sum::<u32>();
        (route, route_distance)
    });
    if let MinMaxResult::MinMax(min, max) = route_distances.minmax_by_key(second) {
        println!("Part 1: {} by path {:?}", min.1, min.0);
        println!("Part 2: {} by path {:?}", max.1, max.0);
    } else {
        panic!("No result");
    }
}

fn main() {
    solve();
}
