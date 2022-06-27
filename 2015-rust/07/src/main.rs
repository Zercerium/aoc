use std::{
    collections::HashMap,
    fs,
    ops::{BitAndAssign, BitOrAssign, Not, Shl, Shr},
};

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Couldnt read Input File");
    let instructions = parse_input(&input);

    let part1 = part_one(&instructions);

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part_two(&instructions, part1));
}

fn parse_input(input: &String) -> HashMap<&str, &str> {
    let mut instructions = HashMap::new();
    for line in input.lines() {
        let line: Vec<&str> = line.split("->").collect();
        assert!(line.len() == 2, "input error");
        if let Some(_) = instructions.insert(line[1].trim(), line[0].trim()) {
            panic!("multiple key detected")
        }
    }

    instructions
}

fn construct_circuit_rec(
    wire: &str,
    instructions: &HashMap<&str, &str>,
    memory: &mut HashMap<String, u16>,
) -> u16 {
    if let Some(n) = memory.get(wire) {
        return *n;
    }
    let i = *instructions
        .get(wire)
        .expect(&format!("wire not found >{}<", wire));

    let return_number;

    match i {
        _ if i.contains("NOT") => {
            let wire = i.rsplit("NOT").next().expect("NOT ERROR").trim();
            return_number = construct_circuit_rec(wire, instructions, memory).not();
        }
        _ if i.contains("RSHIFT") => {
            let mut i = i.split("RSHIFT");
            let wire = i.next().expect("RSHIFT ERROR").trim();
            let number = construct_circuit_rec(wire, instructions, memory);
            let shift_number = i.next().unwrap().trim().parse::<u16>().unwrap();
            return_number = number.shr(shift_number)
        }
        _ if i.contains("LSHIFT") => {
            let mut i = i.split("LSHIFT");
            let wire = i.next().expect("LSHIFT ERROR").trim();
            let number = construct_circuit_rec(wire, instructions, memory);
            let shift_number = i.next().unwrap().trim().parse::<u16>().unwrap();
            return_number = number.shl(shift_number);
        }
        _ if i.contains("AND") => {
            let i = i.split("AND").map(|s| s.trim()).collect::<Vec<_>>();

            let mut nl;
            if let Ok(n) = i[0].parse::<u16>() {
                nl = n;
            } else {
                nl = construct_circuit_rec(i[0], instructions, memory);
            }

            let nr;
            if let Ok(n) = i[1].parse::<u16>() {
                nr = n;
            } else {
                nr = construct_circuit_rec(i[1], instructions, memory);
            }

            nl.bitand_assign(nr);
            return_number = nl;
        }
        _ if i.contains("OR") => {
            let i = i.split("OR").map(|s| s.trim()).collect::<Vec<_>>();

            let mut nl;
            if let Ok(n) = i[0].parse::<u16>() {
                nl = n;
            } else {
                nl = construct_circuit_rec(i[0], instructions, memory);
            }

            let nr;
            if let Ok(n) = i[1].parse::<u16>() {
                nr = n;
            } else {
                nr = construct_circuit_rec(i[1], instructions, memory);
            }

            nl.bitor_assign(nr);
            return_number = nl;
        }
        _ => {
            if let Ok(n) = i.trim().parse::<u16>() {
                return_number = n;
            } else {
                return_number = construct_circuit_rec(i.trim(), instructions, memory);
            }
        }
    }

    memory.insert(wire.to_string(), return_number);
    return_number
}

fn part_one(instructions: &HashMap<&str, &str>) -> u16 {
    let mut memory = HashMap::new();
    construct_circuit_rec("a", &instructions, &mut memory)
}

fn part_two(instructions: &HashMap<&str, &str>, wire_b: u16) -> u16 {
    let mut memory = HashMap::new();
    memory.insert("b".to_string(), wire_b);
    construct_circuit_rec("a", &instructions, &mut memory)
}
