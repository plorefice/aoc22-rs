use std::collections::HashSet;

pub fn part_1(input: &str) -> usize {
    search(input, 4)
}

pub fn part_2(input: &str) -> usize {
    search(input, 14)
}

fn search(input: &str, n: usize) -> usize {
    for (i, win) in input.as_bytes().windows(n).enumerate() {
        if HashSet::<&u8>::from_iter(win).len() == n {
            return i + n;
        }
    }

    unreachable!()
}

crate::solutions! {
    p1 => {
        part_1(include_str!("../inputs/day06.txt")),
        1361
    },
    p2 => {
        part_2(include_str!("../inputs/day06.txt")),
        3263
    }
}
