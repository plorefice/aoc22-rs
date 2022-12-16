use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct Room {
    flow_rate: usize,
    connections: Vec<usize>,
}

pub fn parse_input(input: &str) -> (Vec<Room>, HashMap<String, usize>) {
    let mut rooms = vec![];

    let ids: HashMap<_, _> = input
        .lines()
        .enumerate()
        .map(|(i, l)| (l[6..8].to_string(), i))
        .collect();

    for l in input.lines() {
        let (a, b) = l.split_once("; ").unwrap();

        let flow_rate = a[23..].parse().unwrap();
        let connections = b[23..].split(", ").map(|s| ids[s]).collect();

        rooms.push(Room {
            flow_rate,
            connections,
        })
    }

    (rooms, ids)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    room: usize,
    opened: usize,
    time_left: usize,
    total_rate: usize,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.total_rate.partial_cmp(&other.total_rate)
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub fn part_1((rooms, ids): &(Vec<Room>, HashMap<String, usize>)) -> usize {
    let mut cache = HashSet::new();

    let initial = State {
        room: ids["AA"],
        opened: 0,
        time_left: 30,
        total_rate: 0,
    };

    solve(rooms, &mut cache, initial)
}

fn solve(rooms: &[Room], cache: &mut HashSet<State>, state: State) -> usize {
    if let Some(rate) = cache.get(&state) {
        return rate.total_rate;
    } else {
        cache.insert(state.clone());
    }

    if state.time_left == 0 {
        return state.total_rate;
    }

    let time_left = state.time_left - 1;

    if rooms[state.room].flow_rate > 0 && (state.opened & (1 << state.room)) == 0 {
        solve(
            rooms,
            cache,
            State {
                opened: state.opened | (1 << state.room),
                time_left,
                total_rate: state.total_rate + rooms[state.room].flow_rate * time_left,
                ..state
            },
        )
    } else {
        rooms[state.room]
            .connections
            .iter()
            .filter(|dest| state.opened & (1 << *dest) == 0)
            .map(|dest| {
                solve(
                    rooms,
                    cache,
                    State {
                        room: *dest,
                        time_left,
                        ..state
                    },
                )
            })
            .max()
            .unwrap_or(state.total_rate)
    }
}

crate::solutions! {
    p1 => {
        part_1(&parse_input(include_str!("../inputs/day16.txt"))),
        1923
    }
}
