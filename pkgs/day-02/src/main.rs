use std::str::FromStr;

use nom::{
    character::complete::{alpha1, line_ending, multispace1},
    combinator::{all_consuming, map_opt},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

fn main() {
    let (_, strategy) = parse(include_str!("in").trim()).unwrap();
    dbg!(part1(&strategy));
    dbg!(part2(&strategy));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for Shape {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let letter = s.chars().next();
        match letter {
            Some('A' | 'X') => Ok(Self::Rock),
            Some('B' | 'Y') => Ok(Self::Paper),
            Some('C' | 'Z') => Ok(Self::Scissors),
            _ => Err(()),
        }
    }
}

fn parse(input: &str) -> IResult<&str, Vec<(Shape, Shape)>> {
    all_consuming(separated_list1(
        line_ending,
        map_opt(
            separated_pair(alpha1, multispace1, alpha1),
            |(a, x): (&str, &str)| {
                let a = a.parse::<Shape>().ok()?;
                let x = x.parse::<Shape>().ok()?;
                Some((a, x))
            },
        ),
    ))(input)
}

fn count_score(strategy: &[(Shape, Shape)]) -> u64 {
    strategy
        .iter()
        .map(|turn| {
            use Shape::*;
            match turn {
                (Rock, Rock) => 1 + 3,
                (Rock, Paper) => 2 + 6,
                (Rock, Scissors) => 3 + 0,
                (Paper, Rock) => 1 + 0,
                (Paper, Paper) => 2 + 3,
                (Paper, Scissors) => 3 + 6,
                (Scissors, Rock) => 1 + 6,
                (Scissors, Paper) => 2 + 0,
                (Scissors, Scissors) => 3 + 3,
            }
        })
        .sum()
}

fn part1(strategy: &[(Shape, Shape)]) -> u64 {
    count_score(strategy)
}

fn part2(strategy: &[(Shape, Shape)]) -> u64 {
    let strategy = strategy
        .iter()
        .map(|&turn| {
            use Shape::*;
            match turn {
                (Rock, Rock) => (Rock, Scissors),
                (Paper, Rock) => (Paper, Rock),
                (Scissors, Rock) => (Scissors, Paper),
                (a, Paper) => (a, a),
                (Rock, Scissors) => (Rock, Paper),
                (Paper, Scissors) => (Paper, Scissors),
                (Scissors, Scissors) => (Scissors, Rock),
            }
        })
        .collect::<Vec<_>>();
    count_score(&strategy)
}
