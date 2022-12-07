use std::{collections::VecDeque, path::Path};

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::{
        self,
        complete::{alpha1, newline},
        streaming::line_ending,
    },
    combinator::{all_consuming, map},
    multi::separated_list1,
    sequence::{preceded, separated_pair, terminated},
    IResult,
};

fn main() {
    let (_, commands) = parse(include_str!("in")).unwrap();
    dbg!(part1(&commands));
    dbg!(part2(&commands));
}

#[derive(Debug, Clone)]
enum Command<'a> {
    Cd(&'a str),
    Ls(Vec<Output<'a>>),
}

fn cd(input: &str) -> IResult<&str, Command> {
    map(
        preceded(
            tag("$ cd "),
            take_while(|c: char| c == '/' || c == '.' || c.is_ascii_alphabetic()),
        ),
        Command::Cd,
    )(input)
}

fn ls(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("$ ls")(input)?;
    let (input, _) = newline(input)?;
    let (input, output) = output(input)?;
    Ok((input, Command::Ls(output)))
}

#[derive(Debug, Clone, Copy)]
enum Output<'a> {
    Dir(&'a str),
    File(u128, &'a str),
}

fn dir(input: &str) -> IResult<&str, &str> {
    preceded(tag("dir "), alpha1)(input)
}

fn file(input: &str) -> IResult<&str, (u128, &str)> {
    separated_pair(
        character::complete::u128,
        tag(" "),
        take_while(|c: char| c == '.' || c.is_ascii_alphabetic()),
    )(input)
}

fn output(input: &str) -> IResult<&str, Vec<Output>> {
    separated_list1(
        line_ending,
        alt((
            map(dir, Output::Dir),
            map(file, |(size, name)| Output::File(size, name)),
        )),
    )(input)
}

fn command(input: &str) -> IResult<&str, Command> {
    alt((cd, ls))(input)
}

fn parse(input: &str) -> IResult<&str, Vec<Command>> {
    all_consuming(terminated(
        separated_list1(line_ending, command),
        line_ending,
    ))(input)
}

#[derive(Debug, Clone)]
struct Directory {
    name: String,
    size: u128,
}

fn build_directories(commands: &[Command]) -> Vec<Directory> {
    let mut stack = VecDeque::<Directory>::new();
    let mut directories = Vec::<Directory>::new();
    for command in commands {
        match command {
            Command::Ls(output) => {
                let files: u128 = output
                    .iter()
                    .filter_map(|&out| match out {
                        Output::Dir(_) => None,
                        Output::File(size, _) => Some(size),
                    })
                    .sum();
                if let Some(parent) = stack.back_mut() {
                    parent.size += files;
                }
            }
            Command::Cd(dir) => match *dir {
                ".." => {
                    if let Some(child) = stack.pop_back() {
                        if let Some(parent) = stack.back_mut() {
                            parent.size += child.size;
                        }
                        directories.push(child);
                    }
                }
                name => {
                    let name = Path::new("")
                        .join(stack.back().map_or("", |d| &d.name))
                        .join(name)
                        .to_str()
                        .unwrap()
                        .to_owned();
                    stack.push_back(Directory { name, size: 0 })
                }
            },
        }
    }

    while let Some(child) = stack.pop_back() {
        if let Some(parent) = stack.back_mut() {
            parent.size += child.size;
        }
        directories.push(child);
    }

    directories
}

fn part1(commands: &[Command]) -> u128 {
    let directories = build_directories(commands);
    directories
        .iter()
        .filter_map(|directory| {
            if directory.size <= 100_000 {
                Some(directory.size)
            } else {
                None
            }
        })
        .sum()
}

fn part2(commands: &[Command]) -> Option<u128> {
    let directories = build_directories(commands);
    let root = directories.iter().find(|d| d.name == "/").unwrap();
    let overflow = root.size - (70_000_000 - 30_000_000);
    directories
        .iter()
        .filter_map(|directory| {
            if directory.size >= overflow {
                Some(directory.size)
            } else {
                None
            }
        })
        .min()
}
