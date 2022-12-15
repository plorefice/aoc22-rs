use std::collections::HashSet;

#[derive(Debug, Default)]
pub struct Map {
    sensors: Vec<(Position, isize)>,
    beacons: HashSet<Position>,
}

type Position = (isize, isize);

pub fn parse_input(input: &str) -> Map {
    let mut map = Map::default();

    for l in input.lines() {
        let (a, b) = l.split_once(": ").unwrap();
        let (a, b) = (&a[10..], &b[21..]);
        let (x0, y0) = a.split_once(", ").unwrap();
        let (x1, y1) = b.split_once(", ").unwrap();

        let sensor @ (x0, y0) = (
            x0.split_once('=').unwrap().1.parse::<isize>().unwrap(),
            y0.rsplit_once('=').unwrap().1.parse::<isize>().unwrap(),
        );

        let beacon @ (x1, y1) = (
            x1.split_once('=').unwrap().1.parse::<isize>().unwrap(),
            y1.rsplit_once('=').unwrap().1.parse::<isize>().unwrap(),
        );

        let range = (x1 - x0).abs() + (y1 - y0).abs();

        map.sensors.push((sensor, range));
        map.beacons.insert(beacon);
    }

    map
}

pub fn part_1(map: &Map) -> usize {
    let y_tgt = 2000000;

    let xmin = map
        .sensors
        .iter()
        .map(|((x, y), n)| x - (n - (y_tgt - y).abs()))
        .min()
        .unwrap();

    let xmax = map
        .sensors
        .iter()
        .map(|((x, y), n)| x + (n - (y_tgt - y).abs()))
        .max()
        .unwrap();

    let beacons_on_tgt = map.beacons.iter().filter(|&&(_, y)| y == y_tgt).count();

    (xmin..=xmax)
        .filter(|&x| {
            map.sensors
                .iter()
                .any(|&((sx, sy), n)| (x - sx).abs() + (y_tgt - sy).abs() <= n)
        })
        .count()
        - beacons_on_tgt
}

pub fn part_2(map: &Map) -> isize {
    for y in 0..=4000000 {
        let mut covered = Vec::with_capacity(map.sensors.len());

        for &((sx, sy), n) in &map.sensors {
            let n = n - (sy - y).abs();
            if n < 0 {
                continue;
            }

            covered.push((sx - n, sx + n));
        }

        covered.sort_by_key(|&(s, _)| s);

        let (_, mut xe) = covered[0];
        for &(a, b) in covered.iter().skip(1) {
            if a > xe {
                return (xe + 1) * 4000000 + y;
            }
            xe = xe.max(b);
        }
    }

    unreachable!()
}

crate::solutions! {
    p1 => {
        part_1(&parse_input(include_str!("../inputs/day15.txt"))),
        4502208
    },
    p2 => {
        part_2(&parse_input(include_str!("../inputs/day15.txt"))),
        13784551204480
    }
}
