use nom::{
    bytes::complete::take,
    character::complete::line_ending,
    combinator::map_res,
    multi::{many1, separated_list1},
    IResult,
};

fn main() {
    let (_, trees) = parse(include_str!("in").trim_end()).unwrap();
    dbg!(part1(&trees));
    dbg!(part2(&trees));
}

fn tree(input: &str) -> IResult<&str, usize> {
    map_res(take(1u32), |c: &str| c.parse::<usize>())(input)
}

fn parse(input: &str) -> IResult<&str, Vec<Vec<usize>>> {
    separated_list1(line_ending, many1(tree))(input)
}

fn part1(trees: &[Vec<usize>]) -> usize {
    let size = trees.len() - 1;
    let mut visible_trees: Vec<Vec<bool>> = trees
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, _)| y == 0 || y == size || x == 0 || x == size)
                .collect()
        })
        .collect();

    for y in 0..trees.len() {
        let mut current = trees[y][0];
        for x in 1..trees[0].len() {
            if trees[y][x] > current {
                current = trees[y][x];
                visible_trees[y][x] = true;
            }
        }
    }

    for y in (0..trees.len()).rev() {
        let mut current = trees[y][trees.len() - 1];
        for x in (0..trees[0].len() - 1).rev() {
            if trees[y][x] > current {
                current = trees[y][x];
                visible_trees[y][x] = true;
            }
        }
    }

    for x in 0..trees.len() {
        let mut current = trees[0][x];
        for y in 1..trees[0].len() {
            if trees[y][x] > current {
                current = trees[y][x];
                visible_trees[y][x] = true;
            }
        }
    }

    for x in (0..trees.len()).rev() {
        let mut current = trees[trees.len() - 1][x];
        for y in (0..trees[0].len() - 1).rev() {
            if trees[y][x] > current {
                current = trees[y][x];
                visible_trees[y][x] = true;
            }
        }
    }

    visible_trees.iter().flatten().filter(|&&v| v).count()
}

fn part2(trees: &[Vec<usize>]) -> usize {
    let size = trees.len();
    trees
        .iter()
        .enumerate()
        .flat_map(move |(y, column)| {
            column.iter().enumerate().map(move |(x, &height)| {
                [
                    (0..x).rev().take_while(|&xx| trees[y][xx] < height).count(),
                    ((x + 1)..size)
                        .take_while(|&xx| trees[y][xx] < height)
                        .count(),
                    (0..y).rev().take_while(|&yy| trees[yy][x] < height).count(),
                    ((y + 1)..size)
                        .take_while(|&yy| trees[yy][x] < height)
                        .count(),
                ]
                .iter()
                .product()
            })
        })
        .max()
        .unwrap()
}
