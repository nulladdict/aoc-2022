use std::collections::HashSet;

use nom::{
    character::{complete::alpha1, complete::line_ending},
    combinator::{all_consuming, map},
    multi::separated_list1,
    IResult,
};

fn main() {
    let (_, rucksacks) = parse(include_str!("in").trim()).unwrap();
    dbg!(part1(&rucksacks));
    dbg!(part2(&rucksacks));
}

fn parse(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    all_consuming(separated_list1(
        line_ending,
        map(alpha1, |rucksack: &str| {
            rucksack.chars().collect::<Vec<_>>()
        }),
    ))(input)
}

fn get_cost(letter: char) -> u64 {
    (letter.to_ascii_lowercase() as u64) - 'a' as u64
        + if letter.is_ascii_uppercase() { 27 } else { 1 }
}

fn part1(rucksacks: &[Vec<char>]) -> u64 {
    let mut sum = 0;
    for rucksack in rucksacks {
        let (left, right) = rucksack.split_at(rucksack.len() / 2);
        let left = left.iter().copied().collect::<HashSet<char>>();
        let right = right.iter().copied().collect::<HashSet<char>>();
        let common = left.intersection(&right).collect::<Vec<_>>();
        assert_eq!(common.len(), 1);
        sum += get_cost(*common[0]);
    }
    sum
}

fn part2(rucksacks: &[Vec<char>]) -> u64 {
    let mut sum = 0;
    for group in rucksacks.chunks(3) {
        let common = group
            .iter()
            .map(|rucksack| rucksack.iter().copied().collect::<HashSet<char>>())
            .reduce(|x, y| x.intersection(&y).copied().collect::<HashSet<_>>())
            .unwrap();
        assert_eq!(common.len(), 1);
        sum += get_cost(*common.iter().next().unwrap());
    }
    sum
}
