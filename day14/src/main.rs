use itertools::Itertools;
use std::collections::HashMap;
use std::io::{stdin, BufRead};

fn main() {
    solve();
}

fn solve() {
    let stdin = stdin();
    let handle = stdin.lock();
    let lines = handle.lines().map(|l| l.unwrap()).collect::<Vec<_>>();
    let parsed_lines = lines
        .iter()
        .map(|l| {
            let mut parts = l.split_whitespace();
            let reindeer = parts.next().unwrap();
            parts = parts.dropping(2);
            let speed = parts.next().unwrap().parse::<i32>().unwrap();
            parts = parts.dropping(2);
            let time = parts.next().unwrap().parse::<i32>().unwrap();
            parts = parts.dropping(6);
            let rest = parts
                .next()
                .unwrap()
                .split(',')
                .next()
                .unwrap()
                .parse::<i32>()
                .unwrap();
            ((reindeer), (speed, time, rest))
        })
        .collect::<HashMap<_, _>>();
    let mut reindeer_points = HashMap::<&str, i32>::new();
    let mut reindeer_distance_vec = HashMap::<&str, Vec<i32>>::new();
    for (reindeer, _) in parsed_lines.iter() {
        reindeer_points.insert(*reindeer, 0);
        reindeer_distance_vec.insert(*reindeer, Vec::new());
    }
    for (reindeer, &(speed, time, rest)) in parsed_lines.iter() {
        let mut distance = 0;
        let mut moving = 0;
        let mut rested = rest;
        for _i in 0..2503 {
            if rested < rest {
                rested += 1;
                if rested == rest {
                    moving = 0;
                }
            } else if moving < time {
                distance += speed;
                moving += 1;
                if moving == time {
                    rested = 0;
                }
            }
            let vec = reindeer_distance_vec.get_mut(reindeer).unwrap();
            vec.push(distance);
        }
    }
    for i in 0..2503 {
        let winning_value = reindeer_distance_vec
            .iter()
            .max_by_key(|(_, v)| v.get(i).unwrap())
            .unwrap()
            .1
            .get(i)
            .unwrap();
        for (reindeer, _) in parsed_lines.iter() {
            if let Some(distance) = reindeer_distance_vec.get(reindeer).unwrap().get(i) {
                if distance == winning_value {
                    if let Some(points) = reindeer_points.get_mut(reindeer) {
                        *points += 1;
                    }
                }
            }
        }
    }
    println!(
        "Part 1: {}",
        reindeer_distance_vec
            .iter()
            .map(|(&reindeer, v)| (reindeer, v.last().unwrap()))
            .max_by_key(|&(_, &distance)| distance)
            .unwrap()
            .1
    );
    println!(
        "Part 2: {:?}",
        reindeer_points.iter().max_by_key(|v| v.1).unwrap().1
    );
}
