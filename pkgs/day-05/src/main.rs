use std::collections::VecDeque;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, char, digit1, line_ending},
    combinator::{all_consuming, map, map_res, value},
    multi::separated_list1,
    sequence::{delimited, tuple},
    IResult,
};

fn main() {
    let (_, (layers, steps)) = parse(include_str!("in").trim_end()).unwrap();
    dbg!(part1(&layers, &steps));
    dbg!(part2(&layers, &steps));
}

fn empty_slot(input: &str) -> IResult<&str, &str> {
    tag("   ")(input)
}

fn a_crate(input: &str) -> IResult<&str, &str> {
    delimited(char('['), alpha1, char(']'))(input)
}

fn slot(input: &str) -> IResult<&str, Slot> {
    map(alt((empty_slot, a_crate)), |s: &str| match s {
        "   " => None,
        cargo => Some(cargo.chars().nth(0).unwrap()),
    })(input)
}

fn layer(input: &str) -> IResult<&str, Vec<Slot>> {
    separated_list1(char(' '), slot)(input)
}

type Slot = Option<char>;
type Slots = Vec<Slot>;

fn layers(input: &str) -> IResult<&str, Vec<Slots>> {
    separated_list1(line_ending, layer)(input)
}

fn numbers(input: &str) -> IResult<&str, ()> {
    value(
        (),
        separated_list1(char(' '), delimited(char(' '), digit1, char(' '))),
    )(input)
}

fn a_number(input: &str) -> IResult<&str, usize> {
    map_res(digit1, |s: &str| s.parse::<usize>())(input)
}

type Step = (usize, usize, usize);

fn step(input: &str) -> IResult<&str, Step> {
    map(
        tuple((
            tag("move "),
            a_number,
            tag(" from "),
            a_number,
            tag(" to "),
            a_number,
        )),
        |(_, count, _, from, _, to)| (count, from - 1, to - 1),
    )(input)
}

fn steps(input: &str) -> IResult<&str, Vec<Step>> {
    separated_list1(line_ending, step)(input)
}

fn parse(input: &str) -> IResult<&str, (Vec<Slots>, Vec<Step>)> {
    let (input, layers) = layers(input)?;
    let (input, _) = delimited(line_ending, numbers, line_ending)(input)?;
    let (input, _) = line_ending(input)?;
    let (input, steps) = all_consuming(steps)(input)?;
    Ok((input, (layers, steps)))
}

fn fill_pallets(layers: &[Slots]) -> Vec<VecDeque<char>> {
    let mut pallets = layers[0]
        .iter()
        .map(|_| VecDeque::with_capacity(layers.len()))
        .collect::<Vec<VecDeque<char>>>();
    for layer in layers {
        for (i, slot) in layer.iter().enumerate() {
            if let Some(slot) = slot {
                pallets[i].push_back(*slot);
            }
        }
    }
    pallets
}

fn part1(layers: &[Slots], steps: &[Step]) -> String {
    let mut pallets = fill_pallets(layers);
    for &(count, from, to) in steps {
        for _ in 0..count {
            let cargo = pallets[from].pop_front().unwrap();
            pallets[to].push_front(cargo);
        }
    }
    pallets
        .iter()
        .map(|pallet| pallet.front().unwrap())
        .collect()
}

fn part2(layers: &[Slots], steps: &[Step]) -> String {
    let mut pallets = fill_pallets(layers);
    for &(count, from, to) in steps {
        let mut stash = Vec::new();
        for _ in 0..count {
            stash.push(pallets[from].pop_front().unwrap());
        }
        stash.reverse();
        for cargo in stash {
            pallets[to].push_front(cargo);
        }
    }
    pallets
        .iter()
        .map(|pallet| pallet.front().unwrap())
        .collect()
}
