use nom::{
    branch::alt,
    bytes::complete::tag,
    character::{self, complete::line_ending},
    combinator::{all_consuming, map},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

fn main() {
    let (_, instructions) = parse(include_str!("in").trim_end()).unwrap();
    dbg!(part1(&instructions));
    dbg!(part2(&instructions));
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Addx(i64),
    Noop,
}

fn noop(input: &str) -> IResult<&str, Instruction> {
    map(tag("noop"), |_| Instruction::Noop)(input)
}

fn addx(input: &str) -> IResult<&str, Instruction> {
    map(
        separated_pair(tag("addx"), tag(" "), character::complete::i64),
        |(_, value)| Instruction::Addx(value),
    )(input)
}

fn parse(input: &str) -> IResult<&str, Vec<Instruction>> {
    all_consuming(separated_list1(line_ending, alt((noop, addx))))(input)
}

fn simulate(instructions: &[Instruction]) -> Vec<i64> {
    let mut x = 1;
    let mut history = Vec::with_capacity(instructions.len());
    for instruction in instructions {
        history.push(x);
        match instruction {
            Instruction::Addx(value) => {
                history.push(x);
                x += value;
            }
            Instruction::Noop => {}
        }
    }
    history
}

fn part1(instructions: &[Instruction]) -> i64 {
    let history = simulate(instructions);
    history
        .iter()
        .enumerate()
        .filter_map(|(i, x)| {
            let i = i as i64 + 1;
            if i % 40 == 20 {
                Some(x * i)
            } else {
                None
            }
        })
        .sum()
}

fn part2(instructions: &[Instruction]) {
    let history = simulate(instructions);
    for line in history.chunks(40) {
        println!(
            "{}",
            line.iter()
                .enumerate()
                .map(|(i, x)| {
                    if (i as i64 % 40 - x).abs() <= 1 {
                        '#'
                    } else {
                        '.'
                    }
                })
                .collect::<String>()
        );
    }
}
