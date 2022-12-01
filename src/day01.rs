pub fn parse_input(input: &str) -> Vec<usize> {
    input
        .split("\n\n")
        .map(|chunk| chunk.lines().map(|l| l.parse::<usize>().unwrap()).sum())
        .collect()
}

pub fn part_1(vs: &[usize]) -> usize {
    *vs.iter().max().unwrap()
}

pub fn part_2(vs: &mut [usize]) -> usize {
    vs.sort();
    vs.iter().rev().take(3).sum()
}

crate::solutions!(
    p1 => {
        part_1(&parse_input(include_str!("../inputs/day01.txt"))),
        71023
    },
    p2 => {
        part_2(&mut parse_input(include_str!("../inputs/day01.txt"))),
        206289
    }
);
