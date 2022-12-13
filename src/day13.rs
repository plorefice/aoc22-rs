use std::{cmp::Ordering, iter::Peekable};

use itertools::{EitherOrBoth, Itertools};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PacketData {
    Int(usize),
    List(Vec<PacketData>),
}

impl PartialOrd for PacketData {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        use EitherOrBoth::*;
        use Ordering::*;
        use PacketData::*;

        match (self, other) {
            (Int(a), Int(b)) => a.partial_cmp(b),
            (x @ Int(_), l @ List(_)) => List(vec![x.clone()]).partial_cmp(l),
            (l @ List(_), x @ Int(_)) => l.partial_cmp(&List(vec![x.clone()])),
            (List(a), List(b)) => a
                .iter()
                .zip_longest(b.iter())
                .map(|x| match x {
                    Both(a, b) => a.partial_cmp(b),
                    Left(_) => Some(Greater),
                    Right(_) => Some(Less),
                })
                .find(|&cmp| cmp != Some(Equal))
                .unwrap_or(Some(Equal)),
        }
    }
}

impl Ord for PacketData {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub fn parse_input(input: &str) -> Vec<(PacketData, PacketData)> {
    let mut packets = Vec::new();

    for chunk in input.split("\n\n") {
        let (left, right) = chunk.split_once('\n').unwrap();
        packets.push((
            parse_list(&mut left.bytes().peekable()),
            parse_list(&mut right.bytes().peekable()),
        ));
    }

    packets
}

fn parse_list<T: Iterator<Item = u8>>(bytes: &mut Peekable<T>) -> PacketData {
    let mut data = Vec::new();

    bytes.next().unwrap(); // skip leading '['

    while let Some(b) = bytes.peek() {
        match b {
            b']' => {
                bytes.next().unwrap(); // skip trailing ']'
                return PacketData::List(data);
            }
            b',' => {
                bytes.next();
            }
            b'[' => data.push(parse_list(bytes)),
            b if b.is_ascii_digit() => data.push(parse_int(bytes)),
            _ => unreachable!("{}", *b as char),
        }
    }

    unreachable!("unterminated list")
}

fn parse_int<T: Iterator<Item = u8>>(bytes: &mut Peekable<T>) -> PacketData {
    let s = bytes
        .peeking_take_while(|c| c.is_ascii_digit())
        .map(|b| b as char)
        .collect::<String>();

    PacketData::Int(s.parse().unwrap())
}

pub fn part_1(packets: &[(PacketData, PacketData)]) -> usize {
    packets
        .iter()
        .enumerate()
        .filter_map(|(i, (a, b))| (a < b).then_some(i + 1))
        .sum()
}

pub fn part_2(packets: &[(PacketData, PacketData)]) -> usize {
    use PacketData::*;

    let mut packets = packets.iter().fold(Vec::new(), |mut v, (a, b)| {
        v.push(a.clone());
        v.push(b.clone());
        v
    });

    packets.push(List(vec![Int(2)]));
    packets.push(List(vec![Int(6)]));

    packets.sort();

    let i0 = packets
        .iter()
        .find_position(|p| {
            let List(l) = p else { return false };
            l == &[Int(2)]
        })
        .unwrap();

    let i1 = packets
        .iter()
        .find_position(|p| {
            let List(l) = p else { return false };
            l == &[Int(6)]
        })
        .unwrap();

    (i0.0 + 1) * (i1.0 + 1)
}

crate::solutions! {
    p1 => {
        part_1(&parse_input(include_str!("../inputs/day13.txt"))),
        5340
    },
    p2 => {
        part_2(&parse_input(include_str!("../inputs/day13.txt"))),
        21276
    }
}
