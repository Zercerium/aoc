use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Couldnt read Input File");

    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));
}

fn part_one(input: &String) -> usize {
    let input = input.parse::<usize>().unwrap() / 10;
    let array_size = input;
    let mut array = vec![0; array_size + 1];

    for i in 1..=array_size {
        let mut index = i;
        while index <= array_size {
            array[index] += i;
            index += i;
        }

        if array[i] >= input {
            return i;
        }
    }

    print!("{:#?}", array);
    panic!("array to small");
}

fn part_two(input: &String) -> usize {
    let input = input.parse::<usize>().unwrap();
    let array_size = input;
    let mut array = vec![0; array_size + 1];

    for i in 1..=array_size {
        let mut index = i;
        let mut visited_houses = 1;
        while visited_houses <= 50 && index <= array_size {
            array[index] += i * 11;
            index += i;
            visited_houses += 1;
        }

        if array[i] >= input {
            return i;
        }
    }

    print!("{:#?}", array);
    panic!("array to small");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        assert_eq!(8, part_one(&"150".to_string()));
        assert_eq!(6, part_one(&"120".to_string()));
    }
}
