#[allow(dead_code)]
use std::fmt::Debug;
use std::io::{self, BufRead};
use std::str::FromStr;

#[derive(Debug)]
enum Instruction {
    TurnOn(usize, usize, usize, usize),
    TurnOff(usize, usize, usize, usize),
    Toggle(usize, usize, usize, usize),
}

// turn off 150,300 through 213,740
// turn on 141,242 through 932,871
// toggle 294,259 through 474,326

impl Instruction {
    fn create<F>(f: F, begin: &str, end: &str) -> Instruction
    where
        F: Fn(usize, usize, usize, usize) -> Self,
    {
        let from: Vec<_> = begin
            .split_terminator(',')
            .map(|v| v.parse::<usize>().unwrap())
            .collect();
        let to: Vec<_> = end
            .split_terminator(',')
            .map(|v| v.parse::<usize>().unwrap())
            .collect();
        (f)(from[0], from[1], to[0], to[1])
    }
}

impl FromStr for Instruction {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, ()> {
        let split = s.split_whitespace().collect::<Vec<_>>();
        let instruction = match split.as_slice() {
            ["turn", "off", begin, "through", end] => {
                Instruction::create(Instruction::TurnOff, begin, end)
            }
            ["turn", "on", begin, "through", end] => {
                Instruction::create(Instruction::TurnOn, begin, end)
            }
            ["toggle", begin, "through", end] => {
                Instruction::create(Instruction::Toggle, begin, end)
            }
            _ => panic!("Invalid: {:?}", split),
        };
        Ok(instruction)
    }
}

fn parse_instruction(a: String) -> Instruction {
    // println!("{:?}", a);
    let instruction = Instruction::from_str(&a).unwrap();
    // println!("{:?}", instruction);
    instruction
}

fn solve() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let lights: Vec<Vec<_>> = vec![vec![0u32; 1000]; 1000];
    let brightness: Vec<Vec<_>> = vec![vec![0u32; 1000]; 1000];
    let mut acc = (lights, brightness);
    acc = handle
        .lines()
        .map(|line| parse_instruction(line.unwrap()))
        .fold(acc, |mut acc, instruction| {
            let (ref mut lights, ref mut brightness) = acc;
            match instruction {
                Instruction::TurnOn(from_x, from_y, to_x, to_y) => {
                    for y in from_y..=to_y {
                        for x in from_x..=to_x {
                            lights[y][x] = 1;
                            brightness[y][x] = brightness[y][x].saturating_add(1);
                        }
                    }
                }
                Instruction::TurnOff(from_x, from_y, to_x, to_y) => {
                    for y in from_y..=to_y {
                        for x in from_x..=to_x {
                            lights[y][x] = 0;
                            brightness[y][x] = brightness[y][x].saturating_sub(1);
                        }
                    }
                }
                Instruction::Toggle(from_x, from_y, to_x, to_y) => {
                    for y in from_y..=to_y {
                        for x in from_x..=to_x {
                            lights[y][x] = if lights[y][x] == 0 { 1 } else { 0 };
                            brightness[y][x] = brightness[y][x].saturating_add(2);
                        }
                    }
                }
            }
            acc
        });
    let count = acc.0.iter().flatten().filter(|&&l| l == 1).count();
    let brightness = acc.1.iter().flatten().fold(0, |acc, value| acc + value);
    println!("Lights on: {}", count);
    println!("Brightness: {}", brightness);
}

fn main() {
    solve();
}
