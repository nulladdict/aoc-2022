use std::collections::HashSet;

use nom::{
    bytes::complete::tag,
    character::{
        self,
        complete::{alpha1, line_ending},
    },
    combinator::{all_consuming, map_opt},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

fn main() {
    let (_, moves) = parse(include_str!("in").trim_end()).unwrap();
    dbg!(part1(&moves));
    dbg!(part2(&moves));
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}
impl Direction {
    fn offset(&self) -> Point {
        match self {
            Self::Up => (0, 1),
            Self::Right => (1, 0),
            Self::Down => (0, -1),
            Self::Left => (-1, 0),
        }
    }
}

type Point = (i32, i32);
type Move = (Direction, u32);

fn parse(input: &str) -> IResult<&str, Vec<Move>> {
    all_consuming(separated_list1(
        line_ending,
        map_opt(
            separated_pair(alpha1, tag(" "), character::complete::u32),
            |(letter, size)| match letter {
                "U" => Some((Direction::Up, size)),
                "R" => Some((Direction::Right, size)),
                "D" => Some((Direction::Down, size)),
                "L" => Some((Direction::Left, size)),
                _ => None,
            },
        ),
    ))(input)
}

fn move_by(left: &mut Point, right: Point) {
    left.0 += right.0;
    left.1 += right.1;
}

fn walk<const LENGTH: usize>(moves: &[Move]) -> usize {
    let mut knots = [(0, 0); LENGTH];
    let mut visited = HashSet::new();
    visited.insert(knots[LENGTH - 1]);
    for &(direction, size) in moves {
        for _ in 0..size {
            move_by(&mut knots[0], direction.offset());
            for i in 1..LENGTH {
                let current = knots[i];
                let previous = knots[i - 1];
                let dx = previous.0 - current.0;
                let dy = previous.1 - current.1;
                if dx.abs() >= 2 || dy.abs() >= 2 {
                    move_by(&mut knots[i], (dx.signum(), dy.signum()));
                }
            }
            visited.insert(*knots.last().unwrap());
        }
    }
    visited.len()
}

fn part1(moves: &[Move]) -> usize {
    walk::<2>(moves)
}

fn part2(moves: &[Move]) -> usize {
    walk::<10>(moves)
}
