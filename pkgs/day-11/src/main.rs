use std::{cell::RefCell, collections::VecDeque};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::{self, complete::newline},
    combinator::{all_consuming, map},
    multi::separated_list1,
    sequence::{delimited, preceded},
    IResult,
};
use num::integer::lcm;

fn main() {
    let (_, monkeys) = parse(include_str!("in").trim_end()).unwrap();
    dbg!(part1(monkeys.clone()));
    dbg!(part2(monkeys));
}

#[derive(Debug, Clone, Copy)]
enum Operator {
    Add,
    Mul,
}

#[derive(Debug, Clone, Copy)]
struct Operation {
    left: Option<u64>,
    operator: Operator,
    right: Option<u64>,
}

impl Operation {
    fn apply(&self, value: u64) -> u64 {
        let l = self.left.unwrap_or(value);
        let r = self.right.unwrap_or(value);
        match self.operator {
            Operator::Add => l + r,
            Operator::Mul => l * r,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Test {
    divisible: u64,
    success: usize,
    failure: usize,
}

#[derive(Debug, Clone)]
struct Monkey {
    items: RefCell<VecDeque<u64>>,
    operation: Operation,
    test: Test,
}

impl Monkey {
    fn inspect(&self, item: u64, safety: u64) -> u64 {
        self.operation.apply(item) / safety
    }

    fn decide(&self, worry: u64) -> usize {
        if worry % self.test.divisible == 0 {
            self.test.success
        } else {
            self.test.failure
        }
    }
}

fn old_or_num(input: &str) -> IResult<&str, Option<u64>> {
    alt((
        map(tag("old"), |_| None),
        map(character::complete::u64, Some),
    ))(input)
}

fn operator(input: &str) -> IResult<&str, Operator> {
    alt((
        map(tag(" * "), |_| Operator::Mul),
        map(tag(" + "), |_| Operator::Add),
    ))(input)
}

fn operation(input: &str) -> IResult<&str, Operation> {
    let (input, _) = tag("  Operation: new = ")(input)?;
    let (input, left) = old_or_num(input)?;
    let (input, operator) = operator(input)?;
    let (input, right) = old_or_num(input)?;
    Ok((
        input,
        Operation {
            left,
            operator,
            right,
        },
    ))
}

fn test(input: &str) -> IResult<&str, Test> {
    let (input, divisible) =
        preceded(tag("  Test: divisible by "), character::complete::u64)(input)?;
    let (input, _) = newline(input)?;
    let (input, left) = preceded(
        tag("    If true: throw to monkey "),
        character::complete::u64,
    )(input)?;
    let (input, _) = newline(input)?;
    let (input, right) = preceded(
        tag("    If false: throw to monkey "),
        character::complete::u64,
    )(input)?;
    Ok((
        input,
        Test {
            divisible,
            success: left as usize,
            failure: right as usize,
        },
    ))
}

fn monkey(input: &str) -> IResult<&str, Monkey> {
    let (input, _idx) = delimited(tag("Monkey "), character::complete::u64, tag(":"))(input)?;
    let (input, _) = newline(input)?;
    let (input, items) = preceded(
        tag("  Starting items: "),
        separated_list1(tag(", "), character::complete::u64),
    )(input)?;
    let (input, _) = newline(input)?;
    let (input, operation) = operation(input)?;
    let (input, _) = newline(input)?;
    let (input, test) = test(input)?;
    Ok((
        input,
        Monkey {
            items: RefCell::new(VecDeque::from(items)),
            operation,
            test,
        },
    ))
}

fn parse(input: &str) -> IResult<&str, Vec<Monkey>> {
    all_consuming(separated_list1(tag("\n\n"), monkey))(input)
}

fn simulate<const ROUNDS: usize>(
    monkeys: Vec<Monkey>,
    safety: u64,
) -> (Vec<Vec<VecDeque<u64>>>, Vec<usize>) {
    let base = monkeys
        .iter()
        .map(|monkey| monkey.test.divisible)
        .fold(1, lcm);
    let mut history = Vec::with_capacity(ROUNDS);
    history.push(
        monkeys
            .iter()
            .map(|monkey| monkey.items.borrow().clone())
            .collect::<Vec<_>>(),
    );
    let mut activity = monkeys.iter().map(|_| 0).collect::<Vec<usize>>();

    for _ in 0..ROUNDS {
        for (i, monkey) in monkeys.iter().enumerate() {
            activity[i] += monkey.items.borrow().len();
            while let Some(item) = monkey.items.borrow_mut().pop_front() {
                let worry = monkey.inspect(item, safety) % base;
                let next = monkey.decide(worry);
                let target = monkeys.get(next).unwrap();
                target.items.borrow_mut().push_back(worry);
            }
        }
        history.push(
            monkeys
                .iter()
                .map(|monkey| monkey.items.borrow().clone())
                .collect(),
        );
    }

    (history, activity)
}

fn part1(monkeys: Vec<Monkey>) -> usize {
    let (_, mut activity) = simulate::<20>(monkeys, 3);
    activity.sort_by(|x, y| y.cmp(x));
    activity.iter().take(2).product()
}

fn part2(monkeys: Vec<Monkey>) -> usize {
    let (_, mut activity) = simulate::<10000>(monkeys, 1);
    activity.sort_by(|x, y| y.cmp(x));
    activity.iter().take(2).product()
}
