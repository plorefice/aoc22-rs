use std::ops::RangeInclusive;

type Pair = (RangeInclusive<usize>, RangeInclusive<usize>);

pub fn parse_input(input: &str) -> Vec<Pair> {
    input
        .lines()
        .map(|l| {
            let (a, b) = l.split_once(',').unwrap();
            (
                a.split_once('-')
                    .map(|(l, r)| l.parse().unwrap()..=r.parse().unwrap())
                    .unwrap(),
                b.split_once('-')
                    .map(|(l, r)| l.parse().unwrap()..=r.parse().unwrap())
                    .unwrap(),
            )
        })
        .collect()
}

pub fn part_1(pairs: &[Pair]) -> usize {
    pairs
        .iter()
        .filter(|(a, b)| {
            (a.start() <= b.start() && a.end() >= b.end())
                || (a.start() >= b.start() && a.end() <= b.end())
        })
        .count()
}

pub fn part_2(pairs: &[Pair]) -> usize {
    pairs
        .iter()
        .filter(|(a, b)| {
            a.contains(b.start())
                || a.contains(b.end())
                || b.contains(a.start())
                || b.contains(a.end())
        })
        .count()
}

crate::solutions! {
    p1 => {
        part_1(&parse_input(include_str!("../inputs/day04.txt"))),
        534
    },
    p2 => {
        part_2(&parse_input(include_str!("../inputs/day04.txt"))),
        841
    }
}
