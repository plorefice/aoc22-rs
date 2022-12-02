use std::{convert::Infallible, fmt::Debug, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shape {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    Lose,
    Draw,
    Win,
}

impl FromStr for Shape {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            _ => unreachable!("invalid input: {s}"),
        }
    }
}

impl FromStr for Outcome {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Self::Lose),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Win),
            _ => unreachable!("invalid input: {s}"),
        }
    }
}

impl Shape {
    pub fn beats(&self) -> Shape {
        match self {
            Shape::Rock => Self::Scissors,
            Shape::Paper => Self::Rock,
            Shape::Scissors => Self::Paper,
        }
    }

    pub fn loses_to(&self) -> Shape {
        match self {
            Shape::Paper => Self::Scissors,
            Shape::Scissors => Self::Rock,
            Shape::Rock => Self::Paper,
        }
    }

    pub fn against(&self, them: &Self) -> Outcome {
        if self == them {
            Outcome::Draw
        } else if self.beats() == *them {
            Outcome::Win
        } else {
            Outcome::Lose
        }
    }

    pub fn score(&self) -> usize {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
}

impl Outcome {
    pub fn agains(&self, them: &Shape) -> Shape {
        match self {
            Outcome::Lose => them.beats(),
            Outcome::Draw => *them,
            Outcome::Win => them.loses_to(),
        }
    }

    pub fn score(&self) -> usize {
        match self {
            Outcome::Lose => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        }
    }
}

pub fn parse_input<T>(input: &str) -> Vec<(Shape, T)>
where
    T: FromStr,
    T::Err: Debug,
{
    input
        .lines()
        .map(|l| {
            l.split_once(' ')
                .map(|(l, r)| (l.parse().unwrap(), r.parse::<T>().unwrap()))
                .unwrap()
        })
        .collect()
}

pub fn part_1(rounds: &[(Shape, Shape)]) -> usize {
    rounds
        .iter()
        .map(|(them, me)| me.against(them).score() + me.score())
        .sum()
}

pub fn part_2(rounds: &[(Shape, Outcome)]) -> usize {
    rounds
        .iter()
        .map(|(them, outcome)| outcome.agains(them).score() + outcome.score())
        .sum()
}

crate::solutions! {
    p1 => {
        part_1(&parse_input(include_str!("../inputs/day02.txt"))),
        9177
    },
    p2 => {
        part_2(&parse_input(include_str!("../inputs/day02.txt"))),
        12111
    }
}
