use scan_fmt::scan_fmt;
use std::collections::HashMap;
use std::fs::read_to_string;

const SCAN_SAMPLE: &str = "{}: {}";
const SCAN_LINE: &str = "Sue {}: {}: {}, {}: {}, {}: {}";

fn main() {
    let args: Vec<_> = std::env::args().collect();
    solve(args.get(1).unwrap(), args.get(2).unwrap());
}

fn parse_sample(filename: &str) -> HashMap<String, i32> {
    let contents = read_to_string(filename).unwrap();
    contents.lines().fold(HashMap::new(), |mut acc, line| {
        if let Ok((k, v)) = scan_fmt!(line, SCAN_SAMPLE, String, i32) {
            acc.insert(k, v);
        }
        acc
    })
}

fn parse_input(filename: &str) -> HashMap<i32, HashMap<String, i32>> {
    let contents = read_to_string(filename).unwrap();
    contents.lines().fold(HashMap::new(), |mut acc, line| {
        if let Ok((n, label1, val1, label2, val2, label3, val3)) =
            scan_fmt!(line, SCAN_LINE, i32, String, i32, String, i32, String, i32)
        {
            let mut inner = HashMap::new();
            inner.insert(label1, val1);
            inner.insert(label2, val2);
            inner.insert(label3, val3);
            acc.insert(n, inner);
        }
        acc
    })
}

fn solve(dna_file: &str, input_file: &str) {
    let sample = parse_sample(dna_file);
    let input = parse_input(input_file);
    let mut aunt_1 = None;
    let mut aunt_2 = None;
    for (a, props) in input {
        let is_aunt = props.iter().all(|(k, v)| sample.get(k).unwrap() == v);
        if is_aunt {
            aunt_1 = Some(a);
        }
        let is_proper_aunt = props.iter().all(|(k, v)| {
            if k == "cats" || k == "trees" {
                sample.get(k).unwrap() < v
            } else if k == "pomeranians" || k == "goldfish" {
                sample.get(k).unwrap() > v
            } else {
                sample.get(k).unwrap() == v
            }
        });

        if is_proper_aunt {
            aunt_2 = Some(a);
        }

        if aunt_1.and(aunt_2).is_some() {
            break;
        }
    }
    println!("part 1: {}", aunt_1.unwrap());
    println!("part 2: {}", aunt_2.unwrap());
}
