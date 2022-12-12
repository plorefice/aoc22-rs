#[derive(Debug)]
pub struct Monkey {
    items: Vec<usize>,
    operation: Operation,
    divisible: usize,
    if_true: usize,
    if_false: usize,
}

#[derive(Debug)]
enum Operation {
    Add(Operand),
    Mul(Operand),
}

#[derive(Debug)]
enum Operand {
    Old,
    Int(usize),
}

pub fn parse_input(input: &str) -> Vec<Monkey> {
    input
        .split("\n\n")
        .map(|chunk| {
            let mut lines = chunk.lines().skip(1);

            let items = {
                let line = lines.next().unwrap();
                line[18..].split(", ").map(|n| n.parse().unwrap()).collect()
            };

            let operation = {
                let line = lines.next().unwrap();
                let mut words = line[23..].split_whitespace();

                let op = words.next().unwrap();
                let rhs = match words.next().unwrap().parse::<usize>() {
                    Ok(n) => Operand::Int(n),
                    Err(_) => Operand::Old,
                };

                match op {
                    "+" => Operation::Add(rhs),
                    "*" => Operation::Mul(rhs),
                    _ => unreachable!(),
                }
            };

            let divisible = lines.next().unwrap()[21..].parse().unwrap();
            let if_true = lines.next().unwrap()[29..].parse().unwrap();
            let if_false = lines.next().unwrap()[30..].parse().unwrap();

            Monkey {
                items,
                operation,
                divisible,
                if_true,
                if_false,
            }
        })
        .collect()
}

pub fn part_1(monkeys: Vec<Monkey>) -> usize {
    solve(monkeys, 20, 3)
}

pub fn part_2(monkeys: Vec<Monkey>) -> usize {
    solve(monkeys, 10_000, 1)
}

fn solve(mut monkeys: Vec<Monkey>, iters: usize, div: usize) -> usize {
    let mut inspections = vec![0; monkeys.len()];

    let lcm = monkeys.iter().map(|m| m.divisible).product::<usize>();

    for _ in 0..iters {
        for i in 0..monkeys.len() {
            for j in 0..monkeys[i].items.len() {
                let item = monkeys[i].items[j];

                let item = match monkeys[i].operation {
                    Operation::Add(Operand::Int(n)) => item + n,
                    Operation::Mul(Operand::Int(n)) => item * n,
                    Operation::Add(Operand::Old) => item + item,
                    Operation::Mul(Operand::Old) => item * item,
                };

                let item = (item / div) % lcm;

                let dest = if item % monkeys[i].divisible == 0 {
                    monkeys[i].if_true
                } else {
                    monkeys[i].if_false
                };

                monkeys[dest].items.push(item);
            }

            inspections[i] += monkeys[i].items.len();
            monkeys[i].items.clear();
        }
    }

    inspections.sort();
    inspections.iter().rev().take(2).product()
}

crate::solutions! {
    p1 => {
        part_1(parse_input(include_str!("../inputs/day11.txt"))),
        99840
    },
    p2 => {
        part_2(parse_input(include_str!("../inputs/day11.txt"))),
        20683044837
    }
}
