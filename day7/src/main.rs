#![allow(dead_code)]

use std::collections::HashMap;
use std::fmt::Debug;
use std::io::{self, BufRead};

use cached::{Cached, cached_key};
use cached::UnboundCache;

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
pub struct Instruction {
    op: Operation,
    wire1: String,
    wire2: String,
}

impl Instruction {
    fn new(op: Operation, wire1: String, wire2: String) -> Self {
        Instruction { op, wire1, wire2 }
    }
}

cached_key! {
RECURSE: UnboundCache<String, u16> = UnboundCache::new();
Key = { wire };

fn recurse(
    instructions: &HashMap<String, Instruction>,
    wire: String,
    instruction: &Instruction
) -> u16 = {
    let op = &instruction.op;
    let wire1 = if instruction.wire1.is_empty() {
        0
    } else {
        match instruction.wire1.parse::<u16>() {
            Result::Ok(value) => value,
            _ => recurse(
                instructions,
                instruction.wire1.clone(),
                instructions.get(&instruction.wire1).unwrap(),
            ),
        }
    };
    let wire2 = if instruction.wire2.is_empty() {
        0
    } else {
        match instruction.wire2.parse::<u16>() {
            Result::Ok(value) => value,
            _ => recurse(
                instructions,
                instruction.wire2.clone(),
                instructions.get(&instruction.wire2).unwrap(),
            ),
        }
    };
    match op {
        Operation::Assign => {
            wire1
        }
        Operation::And => {
            wire1 & wire2
        }
        Operation::Or => {
            wire1 | wire2
        }
        Operation::Not => {
            !wire1
        }
        Operation::LShift => {
            wire1 << wire2
        }
        Operation::RShift => {
            wire1 >> wire2
        }
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
    let result = recurse(
        &instructions,
        "a".to_string(),
        instructions.get("a").unwrap(),
    );
    println!("wire a value: {}", result);
    {
        let mut cache = RECURSE.lock().unwrap();
        cache.cache_clear();
        cache.cache_set("b".to_string(), result);
    }
    println!(
        "wire a value: {}",
        recurse(
            &instructions,
            "a".to_string(),
            instructions.get("a").unwrap(),
        )
    );
}

fn main() {
    solve();
}
