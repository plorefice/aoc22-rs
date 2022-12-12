pub enum Instruction {
    Nop,
    Add(isize),
}

impl Instruction {
    pub fn cycles(&self) -> usize {
        match self {
            Instruction::Nop => 1,
            Instruction::Add(_) => 2,
        }
    }
}

pub fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|l| match l.split_once(' ') {
            None => Instruction::Nop,
            Some((_, n)) => Instruction::Add(n.parse().unwrap()),
        })
        .collect()
}

pub fn part_1(instructions: &[Instruction]) -> isize {
    let mut x = 1_isize;
    let mut cycle = 0;
    let mut strength = 0;

    for instr in instructions {
        for n in 0..instr.cycles() {
            cycle += 1;

            if cycle % 40 == 20 {
                strength += cycle * x;
            }

            if cycle >= 220 {
                return strength;
            }

            if n == instr.cycles() - 1 {
                if let &Instruction::Add(inc) = instr {
                    x += inc;
                }
            }
        }
    }

    strength
}

pub fn part_2(instructions: &[Instruction]) -> String {
    let mut x = 1_isize;
    let mut crt = [' '; 40 * 6];
    let mut cycle = 0;

    'l: for instr in instructions {
        for n in 0..instr.cycles() {
            crt[cycle] = {
                let col = cycle % 40;
                if (x - 1..=x + 1).contains(&(col as isize)) {
                    '#'
                } else {
                    ' '
                }
            };

            cycle += 1;

            if cycle >= 240 {
                break 'l;
            }

            if n == instr.cycles() - 1 {
                if let &Instruction::Add(inc) = instr {
                    x += inc;
                }
            }
        }
    }

    for y in 0..6 {
        for x in 0..40 {
            print!("{}", crt[y * 40 + x]);
        }
        println!()
    }

    "RKAZAJBR".to_string()
}

crate::solutions! {
    p1 => {
        part_1(&parse_input(include_str!("../inputs/day10.txt"))),
        16880
    },
    p2 => {
        part_2(&parse_input(include_str!("../inputs/day10.txt"))),
        "RKAZAJBR"
    }
}
