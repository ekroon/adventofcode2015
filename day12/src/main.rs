use jsondata::{Json, Jsons};
use std::io::{stdin, StdinLock};

fn main() {
    solve();
}

fn sum_value_json(json: Json) -> i32 {
    match json {
        Json::Array(jsons) => jsons.into_iter().map(sum_value_json).sum(),
        Json::Integer(value) => value.integer().unwrap() as i32,
        Json::Object(props) => props
            .into_iter()
            .map(|prop| sum_value_json(prop.value()))
            .sum(),
        _ => 0,
    }
}

fn sum_value_json_filtered(json: Json) -> i32 {
    match json {
        Json::Array(jsons) => jsons.into_iter().map(sum_value_json_filtered).sum(),
        Json::Integer(value) => value.integer().unwrap() as i32,
        Json::Object(props) => {
            let has_red = props
                .iter()
                .map(|p| {
                    if let Some(s) = p.value_ref().string() {
                        &s == "red"
                    } else {
                        false
                    }
                })
                .any(|b| b);
            if !has_red {
                props
                    .into_iter()
                    .map(|prop| sum_value_json_filtered(prop.value()))
                    .sum()
            } else {
                0
            }
        }
        _ => 0,
    }
}

fn solve() {
    let stdin = stdin();
    let iter: Jsons<StdinLock> = stdin.lock().into();
    let mut total = 0;
    let mut total_filtered = 0;
    for json in iter {
        match json {
            Ok(value) => {
                total = sum_value_json(value.clone());
                total_filtered = sum_value_json_filtered(value);
            }
            Err(err) => println!("{:?}", err),
        }
    }

    println!("part 1: {}", total);
    println!("part 2: {}", total_filtered);
}
