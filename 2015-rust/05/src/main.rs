use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Couldnt read Input File");

    println!("Part 1: {}", part_one(&input));

    println!("Part 2: {}", part_two(&input));
}

fn is_string_nice(input: &str) -> bool {
    if input.contains("ab") {
        return false;
    }
    if input.contains("cd") {
        return false;
    }
    if input.contains("pq") {
        return false;
    }
    if input.contains("xy") {
        return false;
    }

    let mut vowels = 0;
    let mut twice_in_a_row = false;
    let mut char_before = '.';
    for c in input.chars() {
        match c {
            'a' => vowels += 1,
            'e' => vowels += 1,
            'i' => vowels += 1,
            'o' => vowels += 1,
            'u' => vowels += 1,
            _ => (),
        }

        if c == char_before {
            twice_in_a_row = true;
        }
        char_before = c;

        if vowels >= 3 && twice_in_a_row {
            return true;
        }
    }
    false
}

fn part_one(input: &String) -> usize {
    input.lines().filter(|&s| is_string_nice(s)).count()
}

fn is_string_nice_two(input: &str) -> bool {
    let mut char_tuples = HashMap::new();
    let mut char_before = '.';
    let mut char_before2 = '.';
    let mut char_tuples_found = false;
    let mut char_triple_found = false;
    for (i, c) in input.chars().enumerate() {
        
        if let Some(v) = char_tuples.insert((char_before,c), i) {
            if i - v > 1 {
                char_tuples_found = true;
            } else {
                char_tuples.insert((char_before,c), v);
            }
        }

        if char_before2 == c {
            char_triple_found = true;
        }

        if char_tuples_found && char_triple_found{
            return true;
        }

        char_before2 = char_before;
        char_before = c;
    }
    false
}

fn part_two(input: &String) -> usize {
    input.lines().filter(|&s| is_string_nice_two(s)).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_string_nice() {
        assert_eq!(true, is_string_nice(&"ugknbfddgicrmopn"));
        assert_eq!(true, is_string_nice(&"aaa"));
        assert_eq!(false, is_string_nice(&"jchzalrnumimnmhp"));
        assert_eq!(false, is_string_nice(&"haegwjzuvuyypxyu"));
        assert_eq!(false, is_string_nice(&"dvszwmarrgswjxmb"));
    }

    #[test]
    fn test_is_string_nice_two() {
        assert_eq!(true, is_string_nice_two(&"qjhvhtzxzqqjkmpb"));
        assert_eq!(true, is_string_nice_two(&"xxyxx"));
        assert_eq!(true, is_string_nice_two(&"aaaa"));
        assert_eq!(false, is_string_nice_two(&"aaa"));
        assert_eq!(false, is_string_nice_two(&"uurcxstgmygtbstg"));
        assert_eq!(false, is_string_nice_two(&"ieodomkazucvgmuy"));
    }
}
