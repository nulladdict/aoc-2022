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
use num::{Complex, Signed};

fn main() {
    let (_, moves) = parse(include_str!("in").trim_end()).unwrap();
    dbg!(part1(&moves));
    dbg!(part2(&moves));
}

trait Chebyshev<T>
where
    T: Clone + Signed + Ord,
{
    fn max_norm(&self) -> T;
}

impl<T: Clone + Signed + Ord> Chebyshev<T> for Complex<T> {
    fn max_norm(&self) -> T {
        std::cmp::max(self.re.abs(), self.im.abs())
    }
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
            Self::Up => Complex::new(0, 1),
            Self::Right => Complex::new(1, 0),
            Self::Down => Complex::new(0, -1),
            Self::Left => Complex::new(-1, 0),
        }
    }
}

type Point = Complex<i32>;
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

fn walk<const LENGTH: usize>(moves: &[Move]) -> usize {
    let mut knots = [Complex::new(0, 0); LENGTH];
    let mut visited = HashSet::new();
    visited.insert(knots[LENGTH - 1]);
    for &(direction, size) in moves {
        for _ in 0..size {
            knots[0] += direction.offset();
            for i in 1..LENGTH {
                let current = knots[i];
                let previous = knots[i - 1];
                let delta = previous - current;
                if delta.max_norm() > 1 {
                    knots[i] += Complex::new(delta.re.signum(), delta.im.signum());
                }
            }
            visited.insert(knots[LENGTH - 1]);
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
