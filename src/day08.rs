pub fn parse_input(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|l| l.bytes().map(|c| (c - b'0') as usize).collect())
        .collect()
}

pub fn part_1(map: &[Vec<usize>]) -> usize {
    let mut visibles = 2 * map.len() + 2 * (map[0].len() - 2);

    for y in 1..map.len() - 1 {
        for x in 1..map[y].len() - 1 {
            let tgt = map[y][x];

            if [
                map[y].iter().take(x).all(|&c| c < tgt),
                map[y].iter().skip(x + 1).all(|&c| c < tgt),
                map.iter().take(y).all(|col| col[x] < tgt),
                map.iter().skip(y + 1).all(|col| col[x] < tgt),
            ]
            .iter()
            .any(|cond| *cond)
            {
                visibles += 1;
            }
        }
    }

    visibles
}

pub fn part_2(map: &[Vec<usize>]) -> usize {
    let mut best_score = 0;

    for y in 1..map.len() - 1 {
        for x in 1..map[y].len() - 1 {
            let tgt = map[y][x];

            let mut w_score = 0;
            for x in (0..x).rev() {
                w_score += 1;
                if map[y][x] >= tgt {
                    break;
                }
            }

            let mut e_score = 0;
            for x in x + 1..map[y].len() {
                e_score += 1;
                if map[y][x] >= tgt {
                    break;
                }
            }

            let mut n_score = 0;
            for y in (0..y).rev() {
                n_score += 1;
                if map[y][x] >= tgt {
                    break;
                }
            }

            let mut s_score = 0;
            for row in map.iter().skip(y + 1) {
                s_score += 1;
                if row[x] >= tgt {
                    break;
                }
            }

            best_score = best_score.max(w_score * e_score * n_score * s_score);
        }
    }

    best_score
}

crate::solutions! {
    p1 => {
        part_1(&parse_input(include_str!("../inputs/day08.txt"))),
        1700
    },
    p2 => {
        part_2(&parse_input(include_str!("../inputs/day08.txt"))),
        470596
    }
}
