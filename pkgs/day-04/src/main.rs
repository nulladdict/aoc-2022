use std::ops::RangeInclusive;

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, line_ending},
    combinator::{all_consuming, map_opt},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

fn main() {
    let (_, sections) = parse(include_str!("in").trim()).unwrap();
    dbg!(part1(&sections));
    dbg!(part2(&sections));
}

fn parse_sections(input: &str) -> IResult<&str, RangeInclusive<u64>> {
    map_opt(
        separated_pair(digit1, tag("-"), digit1),
        |(left, right): (&str, &str)| {
            let left = left.parse::<u64>().ok()?;
            let right = right.parse::<u64>().ok()?;
            Some(left..=right)
        },
    )(input)
}

fn parse(input: &str) -> IResult<&str, Vec<(RangeInclusive<u64>, RangeInclusive<u64>)>> {
    all_consuming(separated_list1(
        line_ending,
        separated_pair(parse_sections, tag(","), parse_sections),
    ))(input)
}

fn part1(sections: &[(RangeInclusive<u64>, RangeInclusive<u64>)]) -> usize {
    sections
        .iter()
        .filter(|(left, right)| {
            left.contains(right.start()) && left.contains(right.end())
                || right.contains(left.start()) && right.contains(left.end())
        })
        .count()
}

fn part2(sections: &[(RangeInclusive<u64>, RangeInclusive<u64>)]) -> usize {
    sections
        .iter()
        .filter(|(left, right)| {
            left.contains(right.start())
                || left.contains(right.end())
                || right.contains(left.start())
                || right.contains(left.end())
        })
        .count()
}
