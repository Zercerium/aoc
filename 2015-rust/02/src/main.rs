use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Couldnt read Input File");

    println!("the should order: {} square feet", part_one(&input));

    println!("bow length: {}", part_two(&input));
}

struct Present {
    length: u32,
    width: u32,
    height: u32,
}

impl Present {
    fn wrapping_paper(&self) -> u32 {
        let top = self.length * self.width;
        let front = self.width * self.height;
        let side = self.height * self.length;

        let smalest = self.length * self.width;

        2 * (top + front + side) + smalest
    }

    fn bow_length(&self) -> u32 {
        let bow = self.width * self.height * self.length;
        let wrap = self.length + self.width;

        bow + (wrap * 2)
    }
}

fn parse(input: &String) -> Vec<Present> {
    input
        .lines()
        .map(|l| {
            let mut dimensions: Vec<u32> =
                l.split('x').map(|s| s.parse::<u32>().unwrap()).collect();
            dimensions.sort_unstable();

            Present {
                length: dimensions[0],
                width: dimensions[1],
                height: dimensions[2],
            }
        })
        .collect()
}

fn part_one(input: &String) -> i32 {
    let presents = parse(input);

    let result = presents.iter().fold(0, |acc, p| acc + p.wrapping_paper());

    result as i32
}

fn part_two(input: &String) -> i32 {
    let presents = parse(input);

    let result = presents.iter().fold(0, |acc, p| acc + p.bow_length());

    result as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        assert_eq!(58, part_one(&"2x3x4".to_string()));
        assert_eq!(43, part_one(&"1x1x10".to_string()));
    }

    #[test]
    fn test_two() {
        assert_eq!(34, part_two(&"2x3x4".to_string()));
        assert_eq!(14, part_two(&"1x1x10".to_string()));
    }
}
