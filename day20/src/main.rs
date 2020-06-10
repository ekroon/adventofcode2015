fn main() {
    solve();
}

const RESULT: i32 = 34000000;
const MAX_HOUSES: usize = (RESULT / 10) as usize;

fn solve() {
    let mut house_points = vec![0; MAX_HOUSES];
    'outer: for i in 0..MAX_HOUSES {
        let current_house = i + 1;
        let mut j = i;
        while j < MAX_HOUSES {
            let update_house = house_points.get_mut(j).unwrap_or_else(|| panic!("{}", j));
            *update_house += (current_house * 10) as i32;
            j += current_house;
        }
        if *house_points.get(i).unwrap() >= RESULT {
            println!("part 1: {}", current_house);
            break 'outer;
        }
    }

    let mut house_points = vec![0; MAX_HOUSES];
    'outer: for i in 0..MAX_HOUSES {
        let current_house = i + 1;
        let mut j = i;
        let mut delivered_houses = 0;
        while j < MAX_HOUSES && delivered_houses < 50 {
            let update_house = house_points.get_mut(j).unwrap_or_else(|| panic!("{}", j));
            *update_house += (current_house * 11) as i32;
            j += current_house;
            delivered_houses += 1;
        }
        if *house_points.get(i).unwrap() >= RESULT {
            println!("part 2: {}", current_house);
            break 'outer;
        }
    }
}
