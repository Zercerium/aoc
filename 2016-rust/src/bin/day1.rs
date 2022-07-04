use std::{collections::HashSet, fs};

fn main() {
    let input = fs::read_to_string("./input/input1.txt").expect("Couldnt read Input File");

    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));
}

#[derive(Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

fn new_direction(d: &Direction, turn: char) -> Direction {
    let mut direction = d.clone();
    match d {
        Direction::North => {
            if turn == 'R' {
                direction = Direction::East;
            } else if turn == 'L' {
                direction = Direction::West;
            }
        }
        Direction::East => {
            if turn == 'R' {
                direction = Direction::South;
            } else if turn == 'L' {
                direction = Direction::North;
            }
        }
        Direction::South => {
            if turn == 'R' {
                direction = Direction::West;
            } else if turn == 'L' {
                direction = Direction::East;
            }
        }
        Direction::West => {
            if turn == 'R' {
                direction = Direction::North;
            } else if turn == 'L' {
                direction = Direction::South;
            }
        }
    }
    direction
}

fn part_one(input: &String) -> usize {
    let mut direction = Direction::North;
    let mut x = 0;
    let mut y = 0;
    for o in input.split(", ") {
        let turn = o.chars().nth(0).unwrap();
        let steps = o[1..].trim().parse::<i16>().unwrap_or(0);

        direction = new_direction(&direction, turn);

        match direction {
            Direction::North => y += steps,
            Direction::East => x += steps,
            Direction::South => y -= steps,
            Direction::West => x -= steps,
        }
    }
    println!("X: {}, Y: {}", x, y);
    (x.abs() + y.abs()) as usize
}

fn part_two(input: &String) -> usize {
    let mut direction = Direction::North;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut found = false;

    let mut visited_locations = HashSet::new();
    visited_locations.insert((x, y));

    'outer: for o in input.split(", ") {
        let turn = o.chars().nth(0).unwrap();
        let steps = o[1..].trim().parse::<i16>().unwrap_or(0);

        direction = new_direction(&direction, turn);

        for _ in 0..steps {
            match direction {
                Direction::North => y += 1,
                Direction::East => x += 1,
                Direction::South => y -= 1,
                Direction::West => x -= 1,
            }
            if !visited_locations.insert((x, y)) {
                found = true;
                break 'outer;
            }
        }
    }
    assert!(found, "Location not Found");
    println!("X: {}, Y: {}", x, y);
    (x.abs() + y.abs()) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        assert_eq!(5, part_one(&"R2, L3".to_string()));
        assert_eq!(2, part_one(&"R2, R2, R2".to_string()));
        assert_eq!(12, part_one(&"R5, L5, R5, R3".to_string()));
    }

    #[test]
    fn test_two() {
        assert_eq!(4, part_two(&"R8, R4, R4, R8".to_string()));
    }
}
