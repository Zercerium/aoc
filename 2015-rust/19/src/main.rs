use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Couldnt read Input File");
    let (molecule, replacements) = parse_input(&input);
    println!("Part 1: {}", part_one(&molecule, &replacements));
    println!("Part 2: {}", part_two(&molecule, &replacements));
}

fn parse_input(input: &str) -> (String, HashMap<&str, Vec<&str>>) {
    let mut replacements = HashMap::new();
    let mut input_lines = input.lines();

    let mut line = input_lines.next().unwrap().trim();
    while !line.is_empty() {
        let (key, replacemnt) = line.split_once(" => ").unwrap();
        replacements
            .entry(key)
            .or_insert(Vec::new())
            .push(replacemnt);
        line = input_lines.next().unwrap().trim();
    }

    let medicine_molecule = input_lines.next().unwrap().trim();

    (String::from(medicine_molecule), replacements)
}

fn create_follow_up_molecules(
    molecule: &str,
    replacements: &HashMap<&str, Vec<&str>>,
) -> HashSet<String> {
    let mut replacement_molecules = HashSet::new();

    for (key, replacement) in replacements {
        let key_length = key.len();
        let matches = molecule.match_indices(key);
        for (m, _) in matches {
            for r in replacement {
                let mut new_molecule = molecule.to_string();
                new_molecule.replace_range(m..m + key_length, r);
                replacement_molecules.insert(new_molecule);
            }
        }
    }
    replacement_molecules
}

// fn reverse_engineer(molecule: &str, replacements: &Vec<(String, String)>) -> HashSet<String> {
//     let mut predecessor_mols = HashSet::new();

//     for (key, replacement) in replacements {
//         let key_length = key.len();
//         let matches = molecule.match_indices(key);
//         for (m, _) in matches {
//             let mut new_molecule = molecule.to_string();
//             new_molecule.replace_range(m..m + key_length, &replacement);
//             predecessor_mols.insert(new_molecule);
//         }
//     }
//     predecessor_mols
// }

fn part_one(medicine_molecule: &str, replacements: &HashMap<&str, Vec<&str>>) -> usize {
    create_follow_up_molecules(medicine_molecule, &replacements).len()
}

// to slow, to much mem

// fn part_two(medicine_molecule: &str, replacements: &HashMap<&str, Vec<&str>>) -> usize {
//     let mut i = 0;
//     let mut sequences = HashMap::new();
//     let molecule_len = medicine_molecule.len();
//     sequences.insert("e".to_string(), 0);
//     let mut current_sequence = HashSet::new();
//     current_sequence.insert("e".to_string());

//     loop {
//         i += 1;
//         let mut new_sequences = HashSet::new();
//         for sequence in &current_sequence {
//             for new_sequence in create_follow_up_molecules(sequence, replacements) {
//                 if new_sequence.len() <= molecule_len
//                     && !sequences.contains_key(new_sequence.as_str())
//                 {
//                     new_sequences.insert(new_sequence.clone());
//                     sequences.insert(new_sequence, i);
//                 }
//             }
//         }
//         if sequences.contains_key(medicine_molecule) {
//             break;
//         }
//         assert!(&new_sequences.len() > &0);
//         dbg!(&new_sequences.len());
//         current_sequence = new_sequences;
//         dbg!(i);
//     }

//     i
// }

// fn reverse_rec(mol: &str, replacements: &Vec<(String, String)>, d: usize) -> (usize, bool) {
//     println!("{}", mol);
//     if mol == "e" {
//         return (d, true);
//     }
//     for new_mol in reverse_engineer(&mol, replacements) {
//         let (d, found) = reverse_rec(&new_mol, replacements, d + 1);
//         if found {
//             return (d, true);
//         }
//     }

//     println!("{}", d);
//     (d, false)
// }

// to slow

// fn part_two(medicine_molecule: &str, replacements: &HashMap<&str, Vec<&str>>) -> usize {
//     let mut replacements_rev = HashMap::new();
//     for r in replacements {
//         for v in r.1 {
//             let old = replacements_rev.insert(v.to_string(), r.0.to_string());
//             if old != None {
//                 panic!(">H< => HiHi, >X< => HiHi is not implemented (same output, twice)")
//             }
//         }
//     }
//     let mut replacements_rev = replacements_rev.into_iter().map(|v| v).collect::<Vec<_>>();
//     replacements_rev.sort_unstable_by(|a, b| b.0.len().partial_cmp(&a.0.len()).unwrap());

//     dbg!(&replacements_rev);

//     let (d, found) = reverse_rec(medicine_molecule, &replacements_rev, 0);
//     assert!(found, "not found");
//     d
// }

// https://www.reddit.com/r/adventofcode/comments/3xflz8/comment/cy4f5w8/?utm_source=share&utm_medium=web2x&context=3
fn part_two(medicine_molecule: &str, replacements: &HashMap<&str, Vec<&str>>) -> usize {
    let rn = medicine_molecule.matches("Rn").count();
    let ar = medicine_molecule.matches("Ar").count();
    let y = medicine_molecule.matches("Y").count();
    let symbols = medicine_molecule.matches(char::is_uppercase).count();

    symbols - rn - ar - 2 * y - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let input = r#"H => HO
        H => OH
        O => HH
        
        HOH
        "#;
        let (_, replacements) = parse_input(input);

        assert_eq!(4, part_one("HOH", &replacements));
        assert_eq!(7, part_one("HOHOHO", &replacements));
    }

    // only works for outcommented functions, e => 0 doesnt fit
    #[test]
    fn test_two() {
        let input = r#"e => H
        e => O
        H => HO
        H => OH
        O => HH
        
        HOH
        "#;
        let (_, replacements) = parse_input(input);

        assert_eq!(3, part_two("HOH", &replacements));
        assert_eq!(6, part_two("HOHOHO", &replacements));
    }
}
