use nom::{
    character::complete::{digit1, line_ending},
    combinator::{all_consuming, map, map_res},
    multi::{count, separated_list1},
    IResult,
};

fn main() {
    let mut elves = parse(include_str!("in").trim()).unwrap().1;

    elves.sort_by(|a, b| b.cmp(a));

    dbg!(part1(&elves));
    dbg!(part2(&elves));
}

fn parse(input: &str) -> IResult<&str, Vec<u64>> {
    all_consuming(separated_list1(
        count(line_ending, 2),
        map(
            separated_list1(
                line_ending,
                map_res(digit1, |food: &str| food.parse::<u64>()),
            ),
            |elf| elf.into_iter().sum(),
        ),
    ))(input)
}

fn part1(elves: &[u64]) -> u64 {
    elves[0]
}

fn part2(elves: &[u64]) -> u64 {
    elves.iter().take(3).sum::<u64>()
}
