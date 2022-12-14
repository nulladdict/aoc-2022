use std::{cmp::max, collections::HashSet};

use nom::{
    bytes::complete::tag,
    character::{self, complete::line_ending},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

fn main() {
    let (_, paths) = parse(include_str!("in").trim_end()).unwrap();
    let (cave, floor) = produce_cave(&paths);
    dbg!(part1(&cave, floor));
    dbg!(part2(&cave, floor));
}

type Point = (i64, i64);

fn parse(input: &str) -> IResult<&str, Vec<Vec<Point>>> {
    all_consuming(separated_list1(
        line_ending,
        separated_list1(
            tag(" -> "),
            separated_pair(character::complete::i64, tag(","), character::complete::i64),
        ),
    ))(input)
}

fn produce_cave(paths: &[Vec<Point>]) -> (HashSet<Point>, i64) {
    let mut cave = HashSet::new();
    let mut floor = 0;
    for path in paths {
        for window in path.windows(2) {
            let (mut x1, mut y1) = window[0];
            let (x2, y2) = window[1];
            floor = max(floor, max(y1, y2));
            let dx = (x2 - x1).signum();
            let dy = (y2 - y1).signum();
            cave.insert((x1, y1));
            while (x1, y1) != (x2, y2) {
                x1 += dx;
                y1 += dy;
                cave.insert((x1, y1));
            }
        }
    }
    (cave, floor)
}

fn simulate(mut cave: HashSet<Point>, floor: i64, breakpoint: i64) -> usize {
    let mut sand = 0;
    loop {
        let (mut x, mut y) = (500, 0);
        while y + 1 != floor {
            if !cave.contains(&(x, y + 1)) {
                y += 1;
            } else if !cave.contains(&(x - 1, y + 1)) {
                x -= 1;
                y += 1;
            } else if !cave.contains(&(x + 1, y + 1)) {
                x += 1;
                y += 1;
            } else {
                break;
            }
        }
        if y == breakpoint {
            break sand;
        }
        cave.insert((x, y));
        sand += 1;
    }
}

fn part1(cave: &HashSet<Point>, floor: i64) -> usize {
    simulate(cave.clone(), floor + 2, floor + 1)
}

fn part2(cave: &HashSet<Point>, floor: i64) -> usize {
    simulate(cave.clone(), floor + 2, 0) + 1
}
