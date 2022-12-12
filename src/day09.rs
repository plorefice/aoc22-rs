use std::collections::HashSet;

pub struct Move(char, usize);

pub fn parse_input(input: &str) -> Vec<Move> {
    input
        .lines()
        .map(|l| {
            let (dir, steps) = l.split_once(' ').unwrap();
            let steps = steps.parse().unwrap();
            Move(dir.chars().next().unwrap(), steps)
        })
        .collect()
}

pub fn part_1(moves: &[Move]) -> usize {
    solve(moves, 2)
}

pub fn part_2(moves: &[Move]) -> usize {
    solve(moves, 10)
}

fn solve(moves: &[Move], n: usize) -> usize {
    let mut rope = vec![(0isize, 0isize); n];
    let mut visited = HashSet::from([*rope.last().unwrap()]);

    for &Move(dir, steps) in moves {
        for _ in 0..steps {
            match dir {
                'U' => rope[0].1 -= 1,
                'D' => rope[0].1 += 1,
                'L' => rope[0].0 -= 1,
                'R' => rope[0].0 += 1,
                _ => unreachable!(),
            }

            for i in 1..rope.len() {
                if (rope[i - 1].1 - rope[i].1).abs() > 1 || (rope[i - 1].0 - rope[i].0).abs() > 1 {
                    rope[i].0 += (rope[i - 1].0 - rope[i].0).signum();
                    rope[i].1 += (rope[i - 1].1 - rope[i].1).signum();
                }
            }

            visited.insert(*rope.last().unwrap());
        }
    }

    visited.len()
}

crate::solutions! {
    p1 => {
        part_1(&parse_input(include_str!("../inputs/day09.txt"))),
        6498
    },
    p2 => {
        part_2(&parse_input(include_str!("../inputs/day09.txt"))),
        2531
    }
}
