type Stacks = Vec<Vec<char>>;
type Instructions = Vec<(usize, usize, usize)>;

pub fn parse_input(input: &str) -> (Stacks, Instructions) {
    let mut input = input.split("\n\n");
    (
        parse_stack(input.next().unwrap()),
        parse_instructions(input.next().unwrap()),
    )
}

fn parse_stack(input: &str) -> Stacks {
    let mut lines = input.lines().rev();

    let indices = lines.next().unwrap();
    let mut stacks = vec![Vec::new(); (indices.len() / 4) + 1];

    for line in lines {
        for (i, stack) in stacks.iter_mut().enumerate() {
            let idx = i * 4 + 1;
            if let Some(c) = line.chars().nth(idx) {
                if !c.is_whitespace() {
                    stack.push(c);
                }
            }
        }
    }

    stacks
}

fn parse_instructions(input: &str) -> Instructions {
    input
        .lines()
        .map(|l| {
            let words = l.split_whitespace().collect::<Vec<_>>();
            (
                words[1].parse().unwrap(),
                words[3].parse().unwrap(),
                words[5].parse().unwrap(),
            )
        })
        .collect()
}

pub fn part_1((mut stacks, instructions): (Stacks, Instructions)) -> String {
    for (n, from, to) in instructions {
        for _ in 0..n {
            let x = stacks[from - 1].pop().unwrap();
            stacks[to - 1].push(x);
        }
    }

    stacks.iter().map(|v| v.last().unwrap()).collect()
}

pub fn part_2((mut stacks, instructions): (Stacks, Instructions)) -> String {
    for (n, from, to) in instructions {
        let len = stacks[from - 1].len();
        let xs = stacks[from - 1].split_off(len - n);
        stacks[to - 1].extend(xs);
    }

    stacks.iter().map(|v| v.last().unwrap()).collect()
}

crate::solutions! {
    p1 => {
        part_1(parse_input(include_str!("../inputs/day05.txt"))),
        "TLNGFGMFN"
    },
    p2 => {
        part_2(parse_input(include_str!("../inputs/day05.txt"))),
        "FGLQJCMBD"
    }
}
