use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Couldnt read Input File");
    let pw = part_one(&input);
    println!("Part 1: {}", pw);
    let pw = part_one(&pw);
    println!("Part 2: {}", pw);
}

fn password_increment(pw: &str) -> String {
    let mut pw = pw.chars().rev();
    let mut new_pw = "".to_string();

    loop {
        let c = pw.next();
        if let None = c {
            break;
            // possible overflow
            // zz -> aa ??
        }
        let c = c.unwrap();
        if c == 'z' {
            new_pw.push('a');
        } else {
            let l = (c as u8 + 1) as char;
            new_pw.push(l);
            break; // no more wrap around
        }
    }

    for c in pw {
        new_pw.push(c);
    }

    new_pw.chars().rev().collect::<String>()
}

fn is_pq_req_fulfilled(pw: &str) -> bool {
    is_pq_req_iol(pw) && is_pq_req_abc(pw) && is_pq_req_aa(pw)
}

fn is_pq_req_abc(pw: &str) -> bool {
    let mut pw = pw.chars();
    let mut last_char = pw.next().unwrap() as u8;
    let mut count = 1;

    for c in pw {
        if c as u8 == last_char + 1 {
            count += 1;
            if count >= 3 {
                return true;
            }
        } else {
            count = 1;
        }
        last_char = c as u8;
    }
    return false;
}

fn is_pq_req_iol(pw: &str) -> bool {
    for c in pw.chars() {
        if c == 'i' || c == 'l' || c == 'o' {
            return false;
        }
    }
    return true;
}

fn is_pq_req_aa(pw: &str) -> bool {
    let mut pw = pw.chars();
    let mut last_char = pw.next().unwrap();
    let mut first_match = ' ';

    for c in pw {
        if c == last_char && c != first_match {
            if first_match != ' ' {
                return true;
            }
            first_match = c;
        }
        last_char = c;
    }
    return false;
}

fn part_one(input: &String) -> String {
    let mut pw = password_increment(&input);
    while !is_pq_req_fulfilled(&pw) {
        pw = password_increment(&pw);
    }
    pw
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_pq_req_aa() {
        assert_eq!(true, is_pq_req_aa("abbceffg"));
        assert_eq!(false, is_pq_req_aa("hijklmmn"));
        assert_eq!(false, is_pq_req_aa("abbcegjk"));
    }

    #[test]
    fn test_is_pq_req_iol() {
        assert_eq!(true, is_pq_req_iol("abbceffg"));
        assert_eq!(false, is_pq_req_iol("hijklmmn"));
    }

    #[test]
    fn test_is_pq_req_abc() {
        assert_eq!(true, is_pq_req_abc("hijklmmn"));
        assert_eq!(false, is_pq_req_abc("abbceffg"));
    }

    #[test]
    fn test_password_increment() {
        assert_eq!("xy", password_increment("xx"));
        assert_eq!("xz", password_increment("xy"));
        assert_eq!("ya", password_increment("xz"));
        assert_eq!("yb", password_increment("ya"));
    }

    #[test]
    fn test_part_one() {
        assert_eq!("abcdffaa", part_one(&"abcdefgh".to_string()));
        assert_eq!("ghjaabcc", part_one(&"ghijklmn".to_string()));
    }
}
