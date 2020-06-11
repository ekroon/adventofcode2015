use std::cmp::max;
use std::collections::HashMap;
use std::fs::read_to_string;

use itertools::Itertools;

use scan_fmt::scan_fmt;

const ITEM_LINE: &str = "{/([a-zA-Z]*( \\+[0-9])*)/} {} {} {}";

const BOSS_DAMAGE: i32 = 8;
const BOSS_HIT_POINTS: i32 = 104;
const BOSS_ARMOR: i32 = 1;
const PLAYER_HIT_POINTS: i32 = 100;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Category {
    Weapons,
    Armor,
    Rings,
}

#[derive(Debug)]
struct Item {
    name: String,
    cost: i32,
    damage: i32,
    armor: i32,
}

fn solve() {
    let shopfile = read_to_string("day21/shop.txt");
    let mut shop: HashMap<Category, Vec<Item>> = HashMap::new();
    if let Ok(shopfile) = shopfile {
        let mut lines = shopfile.lines();
        for category in &[Category::Weapons, Category::Armor, Category::Rings] {
            lines.next();
            let mut category_items = Vec::new();
            while let Ok(item_line) = scan_fmt!(
                lines.next().unwrap_or_default(),
                ITEM_LINE,
                String,
                i32,
                i32,
                i32
            ) {
                category_items.push(Item {
                    name: item_line.0,
                    cost: item_line.1,
                    damage: item_line.2,
                    armor: item_line.3,
                });
            }
            if [Category::Armor, Category::Rings].contains(category) {
                for _ in 0..2 {
                    category_items.push(Item {
                        name: "Dummy".into(),
                        cost: 0,
                        damage: 0,
                        armor: 0,
                    });
                }
            }
            shop.insert(*category, category_items);
        }

        let mut min_found_cost: i32 = 1000000;
        let mut max_found_cost: i32 = 0;
        for weapon in shop.get(&Category::Weapons).unwrap() {
            for armor_piece in shop.get(&Category::Armor).unwrap() {
                for rings in shop.get(&Category::Rings).unwrap().iter().combinations(2) {
                    let damage = weapon.damage
                        + armor_piece.damage
                        + rings.first().unwrap().damage
                        + rings.last().unwrap().damage;
                    let armor = weapon.armor
                        + armor_piece.armor
                        + rings.first().unwrap().armor
                        + rings.last().unwrap().armor;
                    let cost = weapon.cost
                        + armor_piece.cost
                        + rings.first().unwrap().cost
                        + rings.last().unwrap().cost;

                    let boss_damage = max(1, BOSS_DAMAGE - armor);
                    let player_turns = (PLAYER_HIT_POINTS + boss_damage - 1) / boss_damage;
                    let player_damage = max(1, damage - BOSS_ARMOR);
                    let boss_turns = (BOSS_HIT_POINTS + player_damage - 1) / player_damage;

                    if player_turns >= boss_turns && cost < min_found_cost {
                        min_found_cost = cost;
                    }
                    if player_turns < boss_turns && cost > max_found_cost {
                        max_found_cost = cost;
                    }
                }
            }
        }
        println!("part 1: {}", min_found_cost);
        println!("part 2: {}", max_found_cost);
    } else {
        println!("Cannot find shop.txt");
    }
}

fn main() {
    solve();
}
