use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Couldnt read Input File");

    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));
}

fn look_and_say(sequence: &str) -> String {
    let mut sequence = sequence.chars();
    let mut last_char = sequence.next().unwrap();
    let mut last_same_char_count = 1;
    let mut output_sequence = "".to_string();

    for c in sequence {
        if last_char == c {
            last_same_char_count += 1;
        } else {
            output_sequence.push_str(&last_same_char_count.to_string());
            output_sequence.push(last_char);

            last_char = c;
            last_same_char_count = 1;
        }
    }
    output_sequence.push_str(&last_same_char_count.to_string());
    output_sequence.push(last_char);

    output_sequence
}

fn part_one(input: &str) -> usize {
    let mut input = input.to_owned();
    for _ in 0..40 {
        input = look_and_say(input.as_str());
    }
    input.len()
}

fn part_two(input: &String) -> usize {
    let mut input = input.to_owned();
    for _ in 0..50 {
        input = look_and_say(input.as_str());
    }
    input.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_look_and_say() {
        assert_eq!("11", look_and_say(&"1".to_string()));
        assert_eq!("21", look_and_say(&"11".to_string()));
        assert_eq!("1211", look_and_say(&"21".to_string()));
        assert_eq!("111221", look_and_say(&"1211".to_string()));
        assert_eq!("312211", look_and_say(&"111221".to_string()));
    }
}
