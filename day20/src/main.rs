fn main() {
    solve();
}

const RESULT: i32 = 34000000;
const MAX_HOUSES: usize = (RESULT / 10) as usize;

fn solve() {
    let mut house_points = vec![0; MAX_HOUSES];
    for i in 0..MAX_HOUSES {
        let current_house = i + 1;
        let mut j = i;
        while j < MAX_HOUSES {
            let update_house = house_points.get_mut(j).unwrap_or_else(|| panic!("{}", j));
            *update_house += (current_house * 10) as i32;
            j += current_house;
        }
        if *house_points.get(i).unwrap() >= RESULT {
            println!("part 1: {}", current_house);
            return;
        }
        if current_house == 6 {
            println!("{} {} {}", i, j, *house_points.get(i).unwrap());
        }
    }
}
