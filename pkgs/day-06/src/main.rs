use std::collections::HashSet;

fn main() {
    let buffer = parse(include_str!("in").trim());
    dbg!(part1(&buffer));
    dbg!(part2(&buffer));
}

fn parse(input: &str) -> Vec<char> {
    input.chars().collect()
}

fn part1(buffer: &[char]) -> Option<usize> {
    find_unique_slice::<4>(buffer)
}

fn part2(buffer: &[char]) -> Option<usize> {
    find_unique_slice::<14>(buffer)
}

fn find_unique_slice<const SIZE: usize>(buffer: &[char]) -> Option<usize> {
    buffer
        .windows(SIZE)
        .enumerate()
        .find(|(_, chunk)| chunk.iter().collect::<HashSet<_>>().len() == SIZE)
        .map(|(i, _)| i + SIZE)
}
