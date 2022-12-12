use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashSet},
    ops::{Index, IndexMut},
};

#[derive(Debug)]
pub struct Map {
    start: (isize, isize),
    end: (isize, isize),
    heightmap: Heightmap,
}

#[derive(Debug)]
struct Heightmap(Vec<Vec<u8>>);

impl Index<(isize, isize)> for Heightmap {
    type Output = u8;

    fn index(&self, (y, x): (isize, isize)) -> &Self::Output {
        &self.0[y as usize][x as usize]
    }
}

impl IndexMut<(isize, isize)> for Heightmap {
    fn index_mut(&mut self, (y, x): (isize, isize)) -> &mut Self::Output {
        &mut self.0[y as usize][x as usize]
    }
}

impl Heightmap {
    pub fn width(&self) -> isize {
        self.0[0].len() as isize
    }

    pub fn height(&self) -> isize {
        self.0.len() as isize
    }
}

pub fn parse_input(input: &str) -> Map {
    let w = input.find('\n').unwrap();
    let h = input.len() / (w + 1);

    let mut map = Map {
        start: (0, 0),
        end: (0, 0),
        heightmap: Heightmap(vec![vec![b'.'; w]; h]),
    };

    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.bytes().enumerate() {
            let (y, x) = (y as isize, x as isize);

            map.heightmap[(y, x)] = match ch {
                b'S' => {
                    map.start = (y, x);
                    0
                }
                b'E' => {
                    map.end = (y, x);
                    b'z' - b'a'
                }
                _ => ch - b'a',
            };
        }
    }

    map
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Iteration {
    pos: (isize, isize),
    steps: usize,
}

impl PartialOrd for Iteration {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.steps.partial_cmp(&other.steps)
    }
}

impl Ord for Iteration {
    fn cmp(&self, other: &Self) -> Ordering {
        self.steps.cmp(&other.steps)
    }
}

pub fn part_1(map: &Map) -> usize {
    let mut visited = HashSet::new();
    let mut remaining = BinaryHeap::new();

    let start = Iteration {
        pos: map.start,
        steps: 0,
    };

    visited.insert(start);
    remaining.push(Reverse(start));

    while let Some(Reverse(Iteration { pos, steps })) = remaining.pop() {
        if pos == map.end {
            return steps;
        }

        for delta in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let next @ (y, x) = (pos.0 + delta.0, pos.1 + delta.1);

            if (0..map.heightmap.height()).contains(&y)
                && (0..map.heightmap.width()).contains(&x)
                && map.heightmap[next] <= map.heightmap[pos] + 1
            {
                let iter = Iteration {
                    pos: next,
                    steps: steps + 1,
                };

                if visited.insert(iter) {
                    remaining.push(Reverse(iter));
                }
            }
        }
    }

    unreachable!()
}

pub fn part_2(map: &Map) -> usize {
    let mut visited = HashSet::new();
    let mut remaining = BinaryHeap::new();

    let start = Iteration {
        pos: map.end,
        steps: 0,
    };

    visited.insert(start);
    remaining.push(Reverse(start));

    while let Some(Reverse(Iteration { pos, steps })) = remaining.pop() {
        if map.heightmap[pos] == 0 {
            return steps;
        }

        for delta in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let next @ (y, x) = (pos.0 + delta.0, pos.1 + delta.1);

            if (0..map.heightmap.height()).contains(&y)
                && (0..map.heightmap.width()).contains(&x)
                && map.heightmap[pos] <= map.heightmap[next] + 1
            {
                let iter = Iteration {
                    pos: next,
                    steps: steps + 1,
                };

                if visited.insert(iter) {
                    remaining.push(Reverse(iter));
                }
            }
        }
    }

    unreachable!()
}

crate::solutions! {
    p1 => {
        part_1(&parse_input(include_str!("../inputs/day12.txt"))),
        472
    },
    p2 => {
        part_2(&parse_input(include_str!("../inputs/day12.txt"))),
        465
    }
}
