use std::{cmp::Ordering, fs};

fn main() {
    let input = fs::read_to_string("./input/input4.txt").expect("Couldnt read Input File");

    // println!("Part 1: {}", part_one(&input));
    // println!("Part 2: {}", part_two(&input));
    part_two(&input);
}

fn part_one(input: &str) -> usize {
    let mut sum = 0;
    let mut letters = [[0 as u8, 0]; 26];

    for (i, letter) in letters.iter_mut().enumerate() {
        letter[0] = i as u8 + 'a' as u8;
    }

    for line in input.lines() {
        let (left, hash) = line.rsplit_once('[').unwrap();
        let hash = hash.strip_suffix(']').unwrap();
        let (name, checksum) = left.rsplit_once('-').unwrap();
        let checksum = checksum.parse::<u16>().unwrap();
        for c in name.chars() {
            if c == '-' {
                continue;
            }
            let letter_id = (c as u8 - 'a' as u8) as usize;
            letters[letter_id][1] += 1;
        }

        let mut is_real_room = true;
        letters.sort_unstable_by(|a, b| {
            if a[1] > b[1] {
                Ordering::Less
            } else if a[1] < b[1] {
                Ordering::Greater
            } else {
                if a[0] > b[0] {
                    Ordering::Greater
                } else if a[0] < b[0] {
                    Ordering::Less
                } else {
                    Ordering::Equal
                }
            }
        });

        for (i, c) in hash.char_indices() {
            if letters[i][0] != c as u8 {
                is_real_room = false;
                break;
            }
        }

        for (i, letter) in letters.iter_mut().enumerate() {
            letter[0] = i as u8 + 'a' as u8;
            letter[1] = 0;
        }

        if is_real_room {
            sum += checksum as usize;
        }
    }
    sum
}

fn decrypt_room_name(name: &str, checksum: u16) -> String {
    let mut new = "".to_string();

    for c in name.chars() {
        if c == '-' {
            new.push(' ');
            continue;
        }

        let mut c = (c as u8 - 'a' as u8) as u16;
        c = (c + checksum) % ('z' as u8 - 'a' as u8 + 1) as u16;
        let c = c as u8 + 'a' as u8;

        new.push(c as char)
    }

    new
}

fn part_two(input: &str) {
    let mut sum = 0;
    let mut letters = [[0 as u8, 0]; 26];
    let mut north_pole_sector_id = 0;

    for (i, letter) in letters.iter_mut().enumerate() {
        letter[0] = i as u8 + 'a' as u8;
    }

    for line in input.lines() {
        let (left, hash) = line.rsplit_once('[').unwrap();
        let hash = hash.strip_suffix(']').unwrap();
        let (name, checksum) = left.rsplit_once('-').unwrap();
        let checksum = checksum.parse::<u16>().unwrap();
        for c in name.chars() {
            if c == '-' {
                continue;
            }
            let letter_id = (c as u8 - 'a' as u8) as usize;
            letters[letter_id][1] += 1;
        }

        let mut is_real_room = true;
        letters.sort_unstable_by(|a, b| {
            if a[1] > b[1] {
                Ordering::Less
            } else if a[1] < b[1] {
                Ordering::Greater
            } else {
                if a[0] > b[0] {
                    Ordering::Greater
                } else if a[0] < b[0] {
                    Ordering::Less
                } else {
                    Ordering::Equal
                }
            }
        });

        for (i, c) in hash.char_indices() {
            if letters[i][0] != c as u8 {
                is_real_room = false;
                break;
            }
        }

        for (i, letter) in letters.iter_mut().enumerate() {
            letter[0] = i as u8 + 'a' as u8;
            letter[1] = 0;
        }

        if is_real_room {
            sum += checksum as usize;
            let name = decrypt_room_name(name, checksum);
            println!("{}", name);
            if name == "northpole object storage" {
                north_pole_sector_id = checksum;
            }
        }
    }
    assert_ne!(0, north_pole_sector_id, "The room isnt here");
    println!("Part 1: {}", sum);
    println!("Part 2: {}", north_pole_sector_id);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        assert_eq!(123, part_one(&"aaaaa-bbb-z-y-x-123[abxyz]".to_string()));
        assert_eq!(987, part_one(&"a-b-c-d-e-f-g-h-987[abcde]".to_string()));
        assert_eq!(404, part_one(&"not-a-real-room-404[oarel]".to_string()));
        assert_eq!(0, part_one(&"totally-real-room-200[decoy]".to_string()));
    }

    #[test]
    fn test_two() {
        assert_eq!("b", decrypt_room_name("a", 1));
        assert_eq!("a", decrypt_room_name("z", 1));
        assert_eq!("c", decrypt_room_name("a", 2));
        assert_eq!(
            "very encrypted name",
            decrypt_room_name("qzmt-zixmtkozy-ivhz", 343)
        );
    }
}
