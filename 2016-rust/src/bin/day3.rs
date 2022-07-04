use std::fs;

fn main() {
    let input = fs::read_to_string("./input/input3.txt").expect("Couldnt read Input File");

    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));
}

fn part_one(input: &str) -> usize {
    let mut count = 0;
    let mut checked = 0;
    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        } else {
            checked += 1
        }
        let a = line[2..=4].trim().parse::<u16>().unwrap();
        let b = line[7..=10].trim().parse::<u16>().unwrap();
        let c = line[12..=14].trim().parse::<u16>().unwrap();
        if a + b > c && a + c > b && b + c > a {
            count += 1;
        }
    }
    println!("checked: {}", checked);
    count
}

fn part_two(input: &str) -> usize {
    let mut count = 0;
    let mut checked = 0;

    let mut ringbuffer = vec![vec![0; 3]; 3];
    let mut ring_index = 0;

    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }
        checked += 1;

        let r0 = line[2..=4].trim().parse::<u16>().unwrap();
        let r1 = line[7..=10].trim().parse::<u16>().unwrap();
        let r2 = line[12..=14].trim().parse::<u16>().unwrap();

        ringbuffer[0][ring_index] = r0;
        ringbuffer[1][ring_index] = r1;
        ringbuffer[2][ring_index] = r2;

        if ring_index == 2 {
            for v in &ringbuffer {
                if v[0] + v[1] > v[2] && v[0] + v[2] > v[1] && v[1] + v[2] > v[0] {
                    count += 1;
                }
            }
        }

        ring_index = (ring_index + 1) % 3;
    }
    println!("checked: {}", checked);
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        assert_eq!(0, part_one(&"    5   10   25".to_string()));
        assert_eq!(0, part_one(&"".to_string()));
    }

    #[test]
    fn test_two() {
        let i = r#"
    5   10    3
    5  100    3
    5   10    3
"#;

        assert_eq!(2, part_two(i));
    }
}
