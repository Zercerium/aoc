use std::{collections::HashSet, fs};

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Couldnt read Input File");

    println!("Part 1: {}", part_one(&input));

    println!("Part 2: {}", part_two(&input));
}

fn part_one(input: &String) -> i32 {
    let mut x = 0;
    let mut y = 0;

    let mut visited_houses = HashSet::new();
    visited_houses.insert((x, y));

    let mut count = 1;

    for c in input.chars() {
        match c {
            '^' => y += 1,
            '<' => x -= 1,
            '>' => x += 1,
            'v' => y -= 1,
            _ => panic!("not supported char in input"),
        }

        let is_new = visited_houses.insert((x, y));
        if is_new {
            count += 1;
        }
    }
    count
}

fn part_two(input: &String) -> i32 {
    let mut x1 = 0;
    let mut y1 = 0;

    let mut x2 = 0;
    let mut y2 = 0;

    let mut visited_houses = HashSet::new();
    visited_houses.insert((x1, y1));

    let mut count = 1;

    for (i, c) in input.chars().enumerate() {
        let x;
        let y;

        if i % 2 == 0 {
            x = &mut x1;
            y = &mut y1;
        } else {
            x = &mut x2;
            y = &mut y2;
        }

        match c {
            '^' => *y += 1,
            '<' => *x -= 1,
            '>' => *x += 1,
            'v' => *y -= 1,
            _ => panic!("not supported char in input"),
        }

        let is_new = visited_houses.insert((*x, *y));
        if is_new {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        assert_eq!(2, part_one(&">".to_string()));
        assert_eq!(4, part_one(&"^>v<".to_string()));
        assert_eq!(2, part_one(&"^v^v^v^v^v".to_string()));
    }

    #[test]
    fn test_two() {
        assert_eq!(3, part_two(&"^v".to_string()));
        assert_eq!(3, part_two(&"^>v<".to_string()));
        assert_eq!(11, part_two(&"^v^v^v^v^v".to_string()));
    }
}
