use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Couldnt read Input File");

    println!("Final Floor: {}", part_one(&input));

    println!("entered basement: {}", part_two(&input));
}

fn part_one(input: &String) -> i32 {
    let opening_brackets = input.chars().filter(|&c| c == '(').count();
    let closing_brackets = input.chars().filter(|&c| c == ')').count();

    opening_brackets as i32 - closing_brackets as i32
}

fn part_two(input: &String) -> i32 {
    let mut current_floor = 0;
    for (i, c) in input.chars().enumerate() {
        match c {
            '(' => current_floor += 1,
            ')' => current_floor -= 1,
            _ => panic!("not supported char in input"),
        }

        if current_floor == -1 {
            return i as i32 + 1;
        }
    }

    panic!("never entered basement");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        assert_eq!(0, part_one(&"(())".to_string()));
    }

    #[test]
    fn test_two() {
        assert_eq!(1, part_two(&")".to_string()));
        assert_eq!(5, part_two(&"()())".to_string()));
    }
}
