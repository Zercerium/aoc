use std::fs;

use serde_json::Value;

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Couldnt read Input File");

    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));
}

fn part_one(input: &String) -> i32 {
    let mut current_number_string = "".to_string();
    let mut is_last_char_numeric = false;
    let mut count = 0;

    for c in input.chars() {
        if c.is_numeric() {
            current_number_string.push(c);
            is_last_char_numeric = true;
        } else {
            if c == '-' {
                current_number_string.push(c);
            } else if is_last_char_numeric {
                // assumption: two numbers are separated
                count += current_number_string.parse::<i32>().unwrap();
                current_number_string.clear();
            }
            is_last_char_numeric = false
        }
    }

    count
}

fn count_rec(v: &Value) -> (i32, bool) {
    let mut count = 0;
    let mut is_red = false;
    dbg!(v);
    match v {
        Value::Null => (),
        Value::Bool(_) => (),
        Value::Number(n) => count = n.as_i64().unwrap() as i32,
        Value::String(s) => {
            if s.contains("red") {
                is_red = true;
                println!("is red!!!")
            }
        }
        Value::Array(a) => {
            for a in a {
                let (count_temp, _) = count_rec(a);
                count += count_temp;
            }
        }
        Value::Object(o) => {
            for m in o {
                let (count_temp, is_red_temp) = count_rec(m.1);
                if is_red_temp {
                    return (0, false);
                }
                count += count_temp;
            }
        }
    }

    if is_red {
        return (0, true);
    } else {
        dbg!(count);
        return (count, false);
    }
}

fn part_two(input: &String) -> i32 {
    let v: Value = serde_json::from_str(&input).unwrap();
    let (count, _) = count_rec(&v);
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        assert_eq!(6, part_one(&"[1,2,3]".to_string()));
        assert_eq!(6, part_one(&r#"{"a":2,"b":4}"#.to_string()));
        assert_eq!(3, part_one(&r#"[[[3]]]"#.to_string()));
        assert_eq!(3, part_one(&r#"{"a":{"b":4},"c":-1}"#.to_string()));
        assert_eq!(0, part_one(&r#"{"a":[-1,1]}"#.to_string()));
        assert_eq!(0, part_one(&r#"[-1,{"a":1}]"#.to_string()));
        assert_eq!(0, part_one(&r#"[]"#.to_string()));
        assert_eq!(0, part_one(&r#"{}"#.to_string()));
    }

    #[test]
    fn test_two() {
        assert_eq!(6, part_two(&"[1,2,3]".to_string()));
        assert_eq!(4, part_two(&r#"[1,{"c":"red","b":2},3]"#.to_string()));
        assert_eq!(6, part_two(&r#"[1,"red",5]"#.to_string()));
        println!("TEST");
        assert_eq!(
            0,
            part_two(&r#"{"d":"red","e":[1,2,3,4],"f":5}"#.to_string())
        );
    }
}
