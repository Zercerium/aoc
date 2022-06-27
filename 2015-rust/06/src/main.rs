use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Couldnt read Input File");
    let instructions = parse_instructions(&input);

    println!("Part 1: {}", part_one(&instructions));
    println!("Part 2: {}", part_two(&instructions));
}

#[derive(Debug)]
enum InstructionOrder {
    On,
    Off,
    Toggle,
}

#[derive(Debug)]
struct Instruction {
    order: InstructionOrder,
    bottem_left: (usize, usize),
    top_right: (usize, usize),
}

struct Grid {
    grid: Vec<Vec<bool>>,
}

impl Grid {
    fn new(x: usize, y: usize, state: bool) -> Self {
        Self {
            grid: vec![vec![state; y]; x],
        }
    }

    fn set_state(&mut self, instruction: &Instruction) {
        let func = match instruction.order {
            InstructionOrder::On => |_| true,
            InstructionOrder::Off => |_| false,
            InstructionOrder::Toggle => |b: bool| !b,
        };

        for x in instruction.bottem_left.0..instruction.top_right.0 + 1 {
            for y in instruction.bottem_left.1..instruction.top_right.1 + 1 {
                self.grid[x][y] = func(self.grid[x][y]);
            }
        }
    }

    fn count(&self) -> usize {
        self.grid.iter().flatten().filter(|&v| *v).count()
    }
}

fn parse_instructions(input: &String) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    for line in input.lines() {
        let numbers: Vec<&str> = line.split([' ', ',']).collect();
        let numbers: Vec<usize> = numbers
            .iter()
            .filter_map(|&n| n.parse::<usize>().ok())
            .collect();

        let mut x_coordinates = vec![numbers[0], numbers[2]];
        let mut y_coordinates = vec![numbers[1], numbers[3]];

        x_coordinates.sort_unstable();
        y_coordinates.sort_unstable();

        let order;

        if line.contains("toggle") {
            order = InstructionOrder::Toggle;
        } else if line.contains("turn on") {
            order = InstructionOrder::On;
        } else if line.contains("turn off") {
            order = InstructionOrder::Off;
        } else {
            panic!("InstructionOrder not supported");
        }

        instructions.push(Instruction {
            order: order,
            bottem_left: (x_coordinates[0], y_coordinates[0]),
            top_right: (x_coordinates[1], y_coordinates[1]),
        });
    }

    instructions
}

fn part_one(instructions: &Vec<Instruction>) -> usize {
    let mut grid = Grid::new(1000, 1000, false);
    instructions.iter().for_each(|i| grid.set_state(i));
    grid.count()
}

struct Grid2 {
    grid: Vec<Vec<usize>>,
}

impl Grid2 {
    fn new(x: usize, y: usize, state: usize) -> Self {
        Self {
            grid: vec![vec![state; y]; x],
        }
    }

    fn set_state(&mut self, instruction: &Instruction) {
        let func = match instruction.order {
            InstructionOrder::On => |u| u + 1,
            InstructionOrder::Off => |u: usize| u.saturating_sub(1),
            InstructionOrder::Toggle => |u| u + 2,
        };

        for x in instruction.bottem_left.0..instruction.top_right.0 + 1 {
            for y in instruction.bottem_left.1..instruction.top_right.1 + 1 {
                self.grid[x][y] = func(self.grid[x][y]);
            }
        }
    }

    fn count(&self) -> usize {
        self.grid.iter().flatten().sum()
    }
}

fn part_two(instructions: &Vec<Instruction>) -> usize {
    let mut grid = Grid2::new(1000, 1000, 0);
    instructions.iter().for_each(|i| grid.set_state(i));
    grid.count()
}
