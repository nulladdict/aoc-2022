use std::cmp::Ordering;

use serde::Deserialize;

use nom::{
    bytes::complete::{tag, take_while1},
    character::complete::line_ending,
    combinator::{all_consuming, map_opt},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

fn main() {
    let (_, packets) = parse(include_str!("in").trim_end()).unwrap();
    dbg!(part1(&packets));
    dbg!(part2(&packets));
}

#[derive(Debug, Clone, Deserialize, Eq)]
#[serde(untagged)]
enum Packet {
    Value(u64),
    Array(Vec<Packet>),
}

impl Packet {
    fn nest(&self) -> Self {
        Self::Array(vec![self.clone()])
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Packet::Value(left), Packet::Value(right)) => left.partial_cmp(right),
            (Packet::Array(left), Packet::Array(right)) => {
                left.iter().zip(right).find(|(l, r)| l != r).map_or_else(
                    || left.len().partial_cmp(&right.len()),
                    |(l, r)| l.partial_cmp(r),
                )
            }
            (left, Packet::Array(right)) => match &**right {
                [r, ..] if left != r => left.partial_cmp(r),
                _ => 1usize.partial_cmp(&right.len()),
            },
            (Packet::Array(_), Packet::Value(_)) => other.partial_cmp(self).map(Ordering::reverse),
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}

fn packet(input: &str) -> IResult<&str, Packet> {
    map_opt(
        take_while1(|c: char| c.is_numeric() || c == '[' || c == ']' || c == ','),
        |line| serde_json::from_str(line).ok(),
    )(input)
}

fn packet_pair(input: &str) -> IResult<&str, (Packet, Packet)> {
    separated_pair(packet, line_ending, packet)(input)
}

fn parse(input: &str) -> IResult<&str, Vec<(Packet, Packet)>> {
    all_consuming(separated_list1(tag("\n\n"), packet_pair))(input)
}

fn part1(packets: &[(Packet, Packet)]) -> usize {
    packets
        .iter()
        .enumerate()
        .filter_map(|(i, (left, right))| (left < right).then_some(i + 1))
        .sum()
}

fn part2(packets: &[(Packet, Packet)]) -> usize {
    let two = Packet::Value(2).nest().nest();
    let six = Packet::Value(6).nest().nest();
    let mut packets = packets
        .iter()
        .cloned()
        .flat_map(|(x, y)| [x, y])
        .collect::<Vec<Packet>>();
    packets.extend([two.clone(), six.clone()]);
    packets.sort_unstable();
    packets
        .iter()
        .enumerate()
        .filter_map(|(i, p)| (p == &two || p == &six).then_some(i + 1))
        .product()
}
