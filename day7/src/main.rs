#![allow(dead_code)]
use std::collections::{HashMap};
use std::fmt::Debug;
use std::io::{self, BufRead};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Operation {
    Assign,
    And,
    Or,
    LShift,
    RShift,
    Not,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Instruction {
    op: Operation,
    wire1: String,
    wire2: String,
}

impl Instruction {
    fn new(op: Operation, wire1: String, wire2: String) -> Self {
        Instruction { op, wire1, wire2 }
    }
}

fn recurse(
    instructions: &HashMap<String, Instruction>,
    cache: &mut HashMap<String, u16>,
    wire: String,
    instruction: &Instruction,
) -> u16 {
    let op = &instruction.op;
    let wire1 = if let Some(value) = cache.get(&wire) {
        return *value;
    } else if instruction.wire1.is_empty() {
        0
    } else {
        match instruction.wire1.parse::<u16>() {
            Result::Ok(value) => value,
            _ => recurse(
                instructions,
                cache,
                instruction.wire1.clone(),
                instructions.get(&instruction.wire1).unwrap(),
            ),
        }
    };
    let wire2 = if let Some(value) = cache.get(&wire) {
        return *value;
    } else if instruction.wire2.is_empty() {
        0
    } else {
        match instruction.wire2.parse::<u16>() {
            Result::Ok(value) => value,
            _ => recurse(
                instructions,
                cache,
                instruction.wire2.clone(),
                instructions.get(&instruction.wire2).unwrap(),
            ),
        }
    };
    match op {
        Operation::Assign => {
            cache.insert(wire, wire1);
            wire1
        }
        Operation::And => {
            let result = wire1 & wire2;
            cache.insert(wire, result);
            result
        }
        Operation::Or => {
            let result = wire1 | wire2;
            cache.insert(wire, result);
            result
        }
        Operation::Not => {
            let result = !wire1;
            cache.insert(wire, result);
            result
        }
        Operation::LShift => {
            let result = wire1 << wire2;
            cache.insert(wire, result);
            result
        }
        Operation::RShift => {
            let result = wire1 >> wire2;
            cache.insert(wire, result);
            result
        }
    }
}

fn solve() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut instructions: HashMap<String, Instruction> = HashMap::new();
    instructions = handle.lines().fold(instructions, |mut map, line| {
        let line = line.unwrap();
        let split = line.split_whitespace().collect::<Vec<_>>();
        match *split.as_slice() {
            [value, "->", wire] => map.insert(
                wire.to_string(),
                Instruction::new(Operation::Assign, value.to_string(), "".to_string()),
            ),
            [wire1, "AND", wire2, "->", wire] => map.insert(
                wire.to_string(),
                Instruction::new(Operation::And, wire1.to_string(), wire2.to_string()),
            ),
            [wire1, "OR", wire2, "->", wire] => map.insert(
                wire.to_string(),
                Instruction::new(Operation::Or, wire1.to_string(), wire2.to_string()),
            ),
            ["NOT", wire1, "->", wire] => map.insert(
                wire.to_string(),
                Instruction::new(Operation::Not, wire1.to_string(), "".to_string()),
            ),
            [wire1, "LSHIFT", value, "->", wire] => map.insert(
                wire.to_string(),
                Instruction::new(Operation::LShift, wire1.to_string(), value.to_string()),
            ),
            [wire1, "RSHIFT", value, "->", wire] => map.insert(
                wire.to_string(),
                Instruction::new(Operation::RShift, wire1.to_string(), value.to_string()),
            ),
            _ => panic!("{:?}", split),
        };
        map
    });
    // println!("{:#?}", instructions);
    let mut cache = HashMap::new();
    println!(
        "wire a value: {}",
        recurse(
            &instructions,
            &mut cache,
            "a".to_string(),
            instructions.get("a").unwrap(),
        )
    );
    let mut cache = HashMap::new();
    cache.insert("b".to_string(), 46065);
    println!(
        "wire a value: {}",
        recurse(
            &instructions,
            &mut cache,
            "a".to_string(),
            instructions.get("a").unwrap(),
        )
    );
}

fn main() {
    solve();
}
