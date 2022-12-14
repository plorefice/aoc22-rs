use std::collections::HashSet;

#[derive(Debug)]
pub struct Wall {
    tiles: HashSet<(isize, isize)>,
    ymax: isize,
}

pub fn parse_input(input: &str) -> Wall {
    let mut wall = Wall {
        tiles: Default::default(),
        ymax: 0,
    };

    for line in input.lines() {
        let coords = line.split(" -> ").map(|x| {
            let (a, b) = x.split_once(',').unwrap();
            (a.parse::<isize>().unwrap(), b.parse::<isize>().unwrap())
        });

        for ((xs, ys), (xe, ye)) in coords.clone().zip(coords.skip(1)) {
            let dx = (xe - xs).signum();
            let dy = (ye - ys).signum();

            let (mut x, mut y) = (xs, ys);

            while x != xe || y != ye {
                wall.tiles.insert((x, y));
                x += dx;
                y += dy;
            }

            wall.tiles.insert((xe, ye));
            wall.ymax = wall.ymax.max(ys).max(ye);
        }
    }

    wall
}

pub fn part_1(mut wall: Wall) -> isize {
    'l: for count in 0.. {
        let (mut x, mut y) = (500, 0);

        'fall: while y <= wall.ymax {
            for next in [(x, y + 1), (x - 1, y + 1), (x + 1, y + 1)] {
                if !wall.tiles.contains(&next) {
                    (x, y) = next;
                    continue 'fall;
                }
            }

            wall.tiles.insert((x, y));
            continue 'l;
        }

        return count;
    }

    unreachable!()
}

pub fn part_2(mut wall: Wall) -> isize {
    wall.ymax += 2;

    for count in 0.. {
        let (mut x, mut y) = (500, 0);

        'fall: loop {
            if y < wall.ymax - 1 {
                for next in [(x, y + 1), (x - 1, y + 1), (x + 1, y + 1)] {
                    if !wall.tiles.contains(&next) {
                        (x, y) = next;
                        continue 'fall;
                    }
                }
            }

            if (x, y) == (500, 0) {
                return count + 1;
            }

            wall.tiles.insert((x, y));
            break;
        }
    }

    unreachable!()
}

crate::solutions! {
    p1 => {
        part_1(parse_input(include_str!("../inputs/day14.txt"))),
        728
    },
    p2 => {
        part_2(parse_input(include_str!("../inputs/day14.txt"))),
        27623
    }
}
