use std::fs;

use md5::{Digest, Md5};

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Couldnt read Input File");

    let result = part_one(&input);

    println!("Part 1: {}", result.0);

    println!("Part 2: {}", result.1);
}

fn part_one(input: &String) -> (u32, u32) {
    let mut i = 1;
    let mut result = (0, 0);

    loop {
        let hash = Md5::digest((input.to_owned() + &i.to_string()).as_bytes());
        let hash = hash.as_slice();

        if hash[0] == 0 && hash[1] == 0 && hash[2] < 16 {
            if result.0 == 0 {
                result.0 = i;
            }
            if hash[2] == 0 {
                result.1 = i;
                break;
            }
        }
        i += 1;
        if i % 10000 == 0 {
            println!("{}", i);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        assert_eq!(609043, part_one(&"abcdef".to_string()).0);
        assert_eq!(1048970, part_one(&"pqrstuv".to_string()).0);
    }
}
