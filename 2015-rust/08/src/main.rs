use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Couldnt read Input File");

    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));
}

fn part_one(input: &str) -> usize {
    let mut count_code = 0;
    let mut count_memory = 0;
    for line in input.lines() {
        let line = line.trim();
        let length = line.len();
        if length == 0 {
            continue;
        }

        let line = line
            .trim()
            .strip_prefix("\"")
            .unwrap()
            .strip_suffix("\"")
            .unwrap();

        count_code += length;

        let mut s: String = "".to_string();
        let mut code_overhead = 2; // 2 quotes
        for c in line.chars() {
            if s.len() == 2 {
                s.remove(0);
            }
            assert!(s.len() <= 2);
            s += &c.to_string();
            match s.as_str() {
                r#"\""# | r#"\\"# => {
                    code_overhead += 1;
                    s.clear()
                }
                r#"\x"# => {
                    code_overhead += 3;
                    s.clear()
                }
                _ => (),
            }
        }
        count_memory += length - code_overhead;
    }
    count_code - count_memory
}

fn part_two(input: &str) -> usize {
    let mut count_code = 0;
    let mut count_encode = 0;
    for line in input.lines() {
        let line = line.trim();
        let length = line.len();
        if length == 0 {
            continue;
        }

        let line = line
            .trim()
            .strip_prefix("\"")
            .unwrap()
            .strip_suffix("\"")
            .unwrap();

        count_code += length;

        let mut s: String = "".to_string();
        let mut encode_overhead = 4; // 2 quotes
        for c in line.chars() {
            if s.len() == 2 {
                s.remove(0);
            }
            assert!(s.len() <= 2);
            s += &c.to_string();
            match s.as_str() {
                r#"\""# | r#"\\"# => {
                    encode_overhead += 2;
                    s.clear()
                }
                r#"\x"# => {
                    encode_overhead += 1;
                    s.clear()
                }
                _ => (),
            }
        }
        count_encode += length + encode_overhead;
    }
    count_encode - count_code
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_STRING: &str = r#"
    ""
    "abc"
    "aaa\"aaa"
    "\x27"
    "#;

    #[test]
    fn test_one() {
        assert_eq!(2, part_one(r#""nq""#));
        assert_eq!(3, part_one(r#""\\""#));
        assert_eq!(2, part_one(r#""abc""#));
        assert_eq!(5, part_one(r#""\x27""#));
        assert_eq!(0, part_one(r#""#));
        assert_eq!(2, part_one(r#""""#));
        assert_eq!(4, part_one(r#""g\"t\\o""#));
        assert_eq!(12, part_one(TEST_STRING));
    }

    #[test]
    fn test_two() {
        assert_eq!(8 - 4, part_two(r#""nq""#));
        assert_eq!(10 - 4, part_two(r#""\\""#));
        assert_eq!(9 - 5, part_two(r#""abc""#));
        assert_eq!(11 - 6, part_two(r#""\x27""#));
        assert_eq!(0, part_two(r#""#));
        assert_eq!(17 - 9, part_two(r#""g\"t\\o""#));
        assert_eq!(42 - 23, part_two(TEST_STRING));
    }
}
