use std::collections::{HashMap, HashSet};
use std::io::{stdin, BufRead};

fn main() {
    solve();
}

fn step(grid: HashMap<(i32, i32), bool>, stuck: &HashSet<(i32, i32)>) -> HashMap<(i32, i32), bool> {
    let mut new_grid = grid.clone();
    grid.iter().for_each(|((x, y), &is_on)| {
        let neighbours: i32 = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ]
        .iter()
        .map(|(x1, y1)| {
            if let Some(&cell) = grid.get(&&(x + x1, y + y1)) {
                if cell {
                    1
                } else {
                    0
                }
            } else {
                0
            }
        })
        .sum();
        if stuck.contains(&&(*x, *y)) {
            new_grid.insert((*x, *y), true);
        } else if is_on && !(neighbours == 2 || neighbours == 3) {
            new_grid.insert((*x, *y), false);
        } else if !is_on && neighbours == 3 {
            new_grid.insert((*x, *y), true);
        }
    });
    new_grid
}

fn solve() {
    let stdin = stdin();
    let handle = stdin.lock();
    let mut original_grid = HashMap::new();
    for (i, s) in handle.lines().enumerate() {
        for (j, c) in s.unwrap().chars().enumerate() {
            original_grid.insert((i as i32, j as i32), c == '#');
        }
    }
    let mut grid1 = original_grid.clone();
    for _ in 0..100 {
        grid1 = step(grid1, &HashSet::new());
    }
    let on = grid1.values().filter(|c| **c).count();
    println!("part 1: {}", on);

    let mut stuck = HashSet::new();
    stuck.insert((0, 0));
    stuck.insert((99, 99));
    stuck.insert((99, 0));
    stuck.insert((0, 99));
    let mut grid2 = original_grid.clone();
    for cell in stuck.iter() {
        grid2.insert(*cell, true);
    }
    for _ in 0..100 {
        grid2 = step(grid2, &stuck);
    }
    let on = grid2.values().filter(|c| **c).count();
    println!("part 2: {}", on);
}
