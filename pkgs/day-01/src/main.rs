use nom::{
    character::complete::{digit1, line_ending},
    combinator::{map, map_res},
    multi::{count, separated_list1},
    IResult,
};

fn main() {
    let mut elfs = parse(include_str!("in")).unwrap().1;

    elfs.sort_by(|a, b| b.cmp(a));

    dbg!(part1(&elfs));
    dbg!(part2(&elfs));
}

fn parse(input: &str) -> IResult<&str, Vec<u64>> {
    separated_list1(
        count(line_ending, 2),
        map(
            separated_list1(
                line_ending,
                map_res(digit1, |food: &str| food.parse::<u64>()),
            ),
            |elf| elf.into_iter().sum(),
        ),
    )(input)
}

fn part1(elfs: &[u64]) -> u64 {
    elfs[0]
}

fn part2(elfs: &[u64]) -> u64 {
    elfs.iter().take(3).sum::<u64>()
}
