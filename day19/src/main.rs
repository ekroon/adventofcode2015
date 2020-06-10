use scan_fmt::{scan_fmt, scanln_fmt};
use std::collections::HashSet;
use std::io::stdin;
use std::ops::Add;

const REACTION: &str = "{} => {}";

fn main() {
    solve();
}

fn solve() {
    let mut reactions = Vec::<(String, String)>::new();
    while let Ok((from, to)) = scanln_fmt!(REACTION, String, String) {
        reactions.push((from, to));
    }
    let mut input = String::new();
    stdin().read_line(&mut input).expect("NO STRING FOUND");
    // println!("{:?} {}", reactions, input);
    let result: HashSet<String> = reactions
        .iter()
        .flat_map(|(from, to)| {
            let mut results = Vec::<String>::new();
            let mut loop_count = 0;
            let mut find_from = 0;
            while let Some(idx) = &input[find_from..].find(from) {
                loop_count += 1;
                let mut result = input.clone();
                result.replace_range((find_from + *idx)..(find_from + *idx + from.len()), to);
                results.push(result);
                find_from += idx + from.len();
            }
            results
        })
        .collect();
    println!("part 1: {:?}", result.len());

    let mut result = input.clone();
    let mut steps = 0;
    let mut replacements_used = Vec::<(String, String, usize)>::new();
    let mut iterations = 0;
    while &result != "e" {
        if (iterations == 100) {
            panic!("Solutions not found");
        }
        iterations += 1;
        let start = result.clone();
        for (from, to) in &reactions {
            let matches = result.matches(to).count();
            steps += matches;
            result = result.replace(to, &from);
            if matches > 0 {
                replacements_used.push((from.clone(), to.clone(), matches));
            }
        }
        if start == result {
            result = input.clone();
            steps = 0;
            reactions.rotate_left(1);
            replacements_used = Vec::new();
        }
    }
    println!("part 2: {}", steps);
    replacements_used.reverse();
    println!("{:?}", replacements_used);
}
