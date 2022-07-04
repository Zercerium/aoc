use std::{cmp::min, fs};

fn main() {
    let input = fs::read_to_string("./input/input2.txt").expect("Couldnt read Input File");

    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));
}

fn part_one(input: &str) -> usize {
    let keypad = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let boundary = keypad.len() - 1; // quadratic keypad
    let mut x = 1 as usize;
    let mut y = 1 as usize;
    let mut code = 0;
    for line in input.lines() {
        for c in line.trim().chars() {
            match c {
                'U' => y = y.saturating_sub(1),
                'D' => y = min(boundary, y + 1),
                'L' => x = x.saturating_sub(1),
                'R' => x = min(boundary, x + 1),
                _ => panic!("cmd not supported: {}", c),
            }
        }
        code *= 10;
        code += keypad[y][x]
    }

    code
}

fn part_two(input: &str) -> String {
    let keypad = vec![
        vec![' ', ' ', '1', ' ', ' '],
        vec![' ', '2', '3', '4', ' '],
        vec!['5', '6', '7', '8', '9'],
        vec![' ', 'A', 'B', 'C', ' '],
        vec![' ', ' ', 'D', ' ', ' '],
    ];
    let mut boundary = Vec::new();
    for v in &keypad {
        boundary.push(v.len() - 1);
    }
    let mut x = 0 as usize;
    let mut y = 2 as usize;
    let mut code = "".to_string();
    for line in input.lines() {
        for c in line.trim().chars() {
            let ox = x;
            let oy = y;
            match c {
                'U' => y = y.saturating_sub(1),
                'D' => y = min(boundary[x], y + 1),
                'L' => x = x.saturating_sub(1),
                'R' => x = min(boundary[x], x + 1),
                _ => panic!("cmd not supported: {}", c),
            }
            if keypad[y][x] == ' ' {
                x = ox;
                y = oy;
            }
        }
        code.push(keypad[y][x]);
    }

    code
}

#[cfg(test)]
mod tests {
    use super::*;
    fn setup() -> String {
        r#"ULL
    RRDDD
    LURDL
    UUUUD
"#
        .to_string()
    }

    #[test]
    fn test_one() {
        let i = &setup();
        assert_eq!(1985, part_one(i));
    }

    #[test]
    fn test_two() {
        let i = &setup();
        assert_eq!("5DB3", part_two(i));
    }
}
