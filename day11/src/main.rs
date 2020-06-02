fn is_valid_password(s: &str) -> bool {
    let mut three_increment = false;
    let mut has_pair = 0;
    let chars: Vec<char> = s.chars().collect();
    for i in 0..chars.len() - 2 {
        if ['i', 'o', 'l'].contains(chars.get(i).unwrap()) {
            return false;
        }

        let current_char_num = *chars.get(i).unwrap() as u8;
        if current_char_num == (*chars.get(i + 1).unwrap() as u8 - 1)
            && current_char_num == (*chars.get(i + 2).unwrap() as u8 - 2)
        {
            three_increment = true;
        }

        if current_char_num != *chars.get(i + 1).unwrap() as u8
            && *chars.get(i + 1).unwrap() as u8 == *chars.get(i + 2).unwrap() as u8
        {
            has_pair += 1;
        }
    }

    has_pair >= 2 && three_increment
}

fn next_valid_password(s: &str) -> String {
    let mut next_password = s.string_successor();
    while !is_valid_password(&next_password) {
        next_password = next_password.string_successor();
    }
    next_password
}

fn solve() {
    let next_password = next_valid_password("cqjxjnds");
    println!("part 1: {}", next_password);
    println!("part 2: {}", next_valid_password(next_password.as_str()));
}

fn main() {
    solve();
}

fn string_successor_impl(s: &str) -> String {
    let mut result = Vec::new();
    let mut carry = true;

    for c in s.chars().rev() {
        if carry && c == 'z' {
            result.push('a')
        } else if carry {
            result.push(((c as u8) + 1) as char);
            carry = false
        } else {
            result.push(c)
        }
    }

    result.into_iter().rev().collect()
}

trait StringSuccessor {
    fn string_successor(&self) -> String;
}
impl<T> StringSuccessor for T
where
    T: AsRef<str>,
{
    fn string_successor(&self) -> String {
        string_successor_impl(self.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_str_short() {
        assert_eq!("a".string_successor(), "b".to_string());
    }

    #[test]
    fn next_string_short() {
        assert_eq!("a".to_string().string_successor(), "b".to_string());
    }

    #[test]
    fn next_str_long() {
        assert_eq!("azzzz".string_successor(), "baaaa".to_string());
    }

    #[test]
    fn next_string_long() {
        assert_eq!("azzzz".to_string().string_successor(), "baaaa".to_string());
    }

    #[test]
    fn next_complex() {
        assert_eq!("qdrsz".string_successor(), "qdrta".to_string());
    }

    #[test]
    fn check_passwords() {
        assert_eq!(is_valid_password("ghjccbaa"), false);
    }

    #[test]
    fn next_valid_password_test() {
        assert_eq!(next_valid_password("ghijklmn"), "ghjaabcc");
    }
}
