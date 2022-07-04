use std::fs;

fn main() {
    let input = fs::read_to_string("./input/input.txt").expect("Couldnt read Input File");

    println!("Part 1: {}", part_one(&input));
    // println!("Part 2: {}", part_two(&input));
}

fn part_one(input: &str) -> usize {
    todo!()
}

fn part_two(input: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        assert_eq!(0, part_one(&"".to_string()));
        assert_eq!(0, part_one(&"".to_string()));
    }

    #[test]
    fn test_two() {
        assert_eq!(0, part_two(&"".to_string()));
        assert_eq!(0, part_two(&"".to_string()));
    }
}
