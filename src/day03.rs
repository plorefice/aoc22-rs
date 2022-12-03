pub fn parse_input(input: &str) -> Vec<&str> {
    input.lines().collect()
}

pub fn part_1(xs: &[&str]) -> usize {
    let mut sum = 0;

    'l: for x in xs {
        let (l, r) = x.split_at(x.len() / 2);
        for c in l.chars() {
            if r.contains(c) {
                sum += prio(c);
                continue 'l;
            }
        }
    }

    sum
}

pub fn part_2(xs: &[&str]) -> usize {
    let mut sum = 0;

    'l: for group in xs.chunks_exact(3) {
        let (x, y, z) = (group[0], group[1], group[2]);
        for c in x.chars() {
            if y.contains(c) && z.contains(c) {
                sum += prio(c);
                continue 'l;
            }
        }
    }

    sum
}

fn prio(ch: char) -> usize {
    if ch.is_lowercase() {
        (ch as u8 - b'a' + 1) as usize
    } else {
        (ch as u8 - b'A' + 27) as usize
    }
}

crate::solutions! {
    p1 => {
        part_1(&parse_input(include_str!("../inputs/day03.txt"))),
        7826
    },
    p2 => {
        part_2(&parse_input(include_str!("../inputs/day03.txt"))),
        2577
    }
}
